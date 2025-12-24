use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Deprecated as this isn't sound, use [`init_openssl_env_vars`] instead.
#[doc(hidden)]
#[deprecated(note = "this function is not safe, use `init_openssl_env_vars` instead")]
pub fn init_ssl_cert_env_vars() {
    unsafe {
        init_openssl_env_vars();
    }
}
