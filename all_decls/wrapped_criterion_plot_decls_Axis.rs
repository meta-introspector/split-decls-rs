use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A coordinate axis
#[derive(Clone, Copy)]
pub enum Axis {
    /// X axis on the bottom side of the figure
    BottomX,
    /// Y axis on the left side of the figure
    LeftY,
    /// Y axis on the right side of the figure
    RightY,
    /// X axis on the top side of the figure
    TopX,
}
