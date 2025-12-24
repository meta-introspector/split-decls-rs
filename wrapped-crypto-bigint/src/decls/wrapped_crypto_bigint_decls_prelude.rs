use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Import prelude for this crate: includes important traits.
pub mod prelude {
    #[cfg(feature = "hybrid-array")]
    pub use crate::array::{ArrayDecoding, ArrayEncoding};
    pub use crate::traits::*;
}
