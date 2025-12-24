use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(
    any(
        feature = "portable-atomic",
        all(feature = "mpmc_large", target_has_atomic = "ptr"),
        all(not(feature = "mpmc_large"), target_has_atomic = "8")
    )
)]
pub mod mpmc;
