use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "alloc")]
pub mod collections {
    pub use super::raw_vec::{TryReserveError, TryReserveErrorKind};
}
