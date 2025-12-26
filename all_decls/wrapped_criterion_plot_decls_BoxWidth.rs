use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Box width for box-related plots: bars, candlesticks, etc
#[derive(Clone, Copy)]
pub struct BoxWidth(pub f64);
