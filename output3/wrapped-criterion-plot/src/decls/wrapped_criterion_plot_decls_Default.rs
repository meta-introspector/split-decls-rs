use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Not public version of `std::default::Default`, used to not leak default constructors into the
/// public API
trait Default {
    /// Creates `Properties` with default configuration
    fn default() -> Self;
}
