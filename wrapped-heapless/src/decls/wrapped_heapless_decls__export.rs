use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Implementation details for macros.
/// Do not use. Used for macros only. Not covered by semver guarantees.
#[doc(hidden)]
pub mod _export {
    pub use crate::string::format;
}
