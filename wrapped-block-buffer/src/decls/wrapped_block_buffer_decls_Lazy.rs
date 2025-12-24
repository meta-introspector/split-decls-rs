use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Lazy block buffer kind, which guarantees that buffer position
/// always lies in the range of `0..=BlockSize`.
#[derive(Copy, Clone, Debug, Default)]
pub struct Lazy {}
