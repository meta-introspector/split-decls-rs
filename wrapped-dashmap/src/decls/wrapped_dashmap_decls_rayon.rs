use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "rayon")]
pub mod rayon {
    pub mod map;
    pub mod read_only;
    pub mod set;
}
