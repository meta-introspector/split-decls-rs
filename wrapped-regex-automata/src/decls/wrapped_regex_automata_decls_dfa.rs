use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(any(feature = "dfa-search", feature = "dfa-onepass"))]
pub mod dfa;
