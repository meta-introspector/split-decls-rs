use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Deprecated, see [`aot`]
pub mod generator {
    pub use crate::aot::generate;
    pub use crate::aot::generate_to;
    pub use crate::aot::utils;
    pub use crate::aot::Generator;
}
