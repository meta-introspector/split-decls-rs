use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Probe for SSL certificates on the system, then configure the SSL certificate `SSL_CERT_FILE`
/// and `SSL_CERT_DIR` environment variables in this process for OpenSSL to use.
///
/// Preconfigured values in the environment variables will not be overwritten if the paths they
/// point to exist and are accessible.
///
/// Returns `true` if any certificate file or directory was found while probing.
/// Combine this with `has_ssl_cert_env_vars()` to check whether previously configured environment
/// variables are valid.
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
pub unsafe fn try_init_openssl_env_vars() -> bool {
    let ProbeResult {
        cert_file,
        cert_dir,
    } = probe();
    if let Some(path) = &cert_file {
        unsafe {
            put(ENV_CERT_FILE, path);
        }
    }
    if let Some(path) = &cert_dir {
        unsafe {
            put(ENV_CERT_DIR, path);
        }
    }
    unsafe fn put(var: &str, path: &Path) {
        if env::var_os(var).as_deref() != Some(path.as_os_str()) {
            unsafe {
                env::set_var(var, path);
            }
        }
    }
    cert_file.is_some() || cert_dir.is_some()
}
