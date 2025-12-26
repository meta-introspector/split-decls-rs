use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum CountError {
    OutOfBounds,
    Misplaced,
}
