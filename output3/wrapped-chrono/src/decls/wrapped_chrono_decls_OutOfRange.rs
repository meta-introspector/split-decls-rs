use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Out of range error type used in various converting APIs
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct OutOfRange {
    _private: (),
}
