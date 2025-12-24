use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Entry point to the lib. Use this to handle your formatting needs.
#[derive(Debug)]
pub struct Formatter {
    decimals: usize,
    separator: String,
    scales: Scales,
    forced_units: String,
    forced_suffix: String,
}
