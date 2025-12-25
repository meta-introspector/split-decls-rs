use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(anyhow_nightly_testing, feature = "std", not(error_generic_member_access)))]
compile_error!("Build script probe failed to compile.");
