use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Axis range
#[derive(Clone, Copy)]
pub enum Range {
    /// Autoscale the axis
    Auto,
    /// Set the limits of the axis
    Limits(f64, f64),
}
