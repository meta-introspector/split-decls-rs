use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Color
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Blue,
    Cyan,
    DarkViolet,
    ForestGreen,
    Gold,
    Gray,
    Green,
    Magenta,
    Red,
    /// Custom RGB color
    Rgb(u8, u8, u8),
    White,
    Yellow,
}
