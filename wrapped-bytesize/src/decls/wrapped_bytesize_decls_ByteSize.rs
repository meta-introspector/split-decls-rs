use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Byte size representation.
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct ByteSize(pub u64);
