use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Trait implemented for supported block sizes, i.e. for types from `U1` to `U255`.
pub trait BlockSizes: ArraySize + sealed::BlockSizes {}
