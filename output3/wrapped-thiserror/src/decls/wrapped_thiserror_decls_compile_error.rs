use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(thiserror_nightly_testing, not(error_generic_member_access)))]
compile_error!("Build script probe failed to compile.");
