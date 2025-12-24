use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Lazy block buffer.
pub type LazyBuffer<B> = BlockBuffer<B, Lazy>;
