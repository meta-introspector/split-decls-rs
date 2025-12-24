use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(target_arch = "aarch64", target_feature = "neon", not(miri)))]
mod neon;
