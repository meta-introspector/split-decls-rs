use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Trait for buffer kinds.
pub trait BufferKind: sealed::Sealed {}
