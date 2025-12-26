use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(feature = "bilock", not(feature = "unstable")))]
compile_error!(
    "The `bilock` feature requires the `unstable` feature as an explicit opt-in to unstable features"
);
