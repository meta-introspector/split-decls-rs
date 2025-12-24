use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Build a `char` out of bytes
pub trait CharAccumulator: Default {
    /// Build a `char` out of bytes
    ///
    /// Return `None` when more data is needed
    fn add(&mut self, byte: u8) -> Option<char>;
}
