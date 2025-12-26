use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Eager block buffer.
pub type EagerBuffer<B> = BlockBuffer<B, Eager>;
