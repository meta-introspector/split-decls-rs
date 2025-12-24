use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Crate internals used by the `select!` macro.
#[doc(hidden)]
#[cfg(feature = "std")]
pub mod internal {
    pub use crate::select::{select, select_timeout, try_select, SelectHandle};
}
