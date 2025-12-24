use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "disable-signatures"))]
mod edwards25519;
