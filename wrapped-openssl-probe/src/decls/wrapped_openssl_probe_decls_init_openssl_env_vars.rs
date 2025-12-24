use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Probe for SSL certificates on the system, then configure the SSL certificate `SSL_CERT_FILE`
/// and `SSL_CERT_DIR` environment variables in this process for OpenSSL to use.
///
/// Preconfigured values in the environment variables will not be overwritten if the paths they
/// point to exist and are accessible.
///
/// # Safety
///
/// This function is not safe because it mutates the process's environment
/// variables which is generally not safe. See the [documentation in libstd][doc]
/// for information about why setting environment variables is not safe.
///
/// If possible use the [`probe`] function and directly configure OpenSSL
/// methods instead of relying on environment variables.
///
/// [doc]: https://doc.rust-lang.org/stable/std/env/fn.set_var.html#safety
pub unsafe fn init_openssl_env_vars() {
    try_init_openssl_env_vars();
}
