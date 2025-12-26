use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Resettable types.
pub trait Reset {
    /// Reset state to its initial value.
    fn reset(&mut self);
}
