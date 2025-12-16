//! Gradient support for borders and other elements.
//!
//! This module provides types for applying gradients to UI elements, particularly borders.

use alloc::vec::Vec;

use crate::style::Color;

/// Defines gradients for each side of a block's borders.
///
/// Each side (top, bottom, left, right) can have an optional gradient defined as a vector of
/// colors. When rendering, these colors are applied sequentially to the border characters.
///
/// # Examples
///
/// ```
/// use ratatui_core::{gradient::BorderGradients, style::Color};
///
/// let gradients = BorderGradients {
///     top: Some(vec![Color::Red, Color::Yellow, Color::Green]),
///     bottom: Some(vec![Color::Blue, Color::Cyan, Color::White]),
///     left: None,
///     right: None,
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct BorderGradients {
    /// Gradient colors for the top border, from left to right
    pub top: Option<Vec<Color>>,
    /// Gradient colors for the left border, from top to bottom
    pub left: Option<Vec<Color>>,
    /// Gradient colors for the right border, from top to bottom
    pub right: Option<Vec<Color>>,
    /// Gradient colors for the bottom border, from left to right
    pub bottom: Option<Vec<Color>>,
}
