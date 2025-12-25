use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(any(
    windows,
    target_os = "macos",
    target_os = "ios",
    not(feature = "https")
))]
fn openssl_env_init() {}
