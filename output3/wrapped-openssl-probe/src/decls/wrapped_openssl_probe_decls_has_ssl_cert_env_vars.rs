use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Check whether the OpenSSL `SSL_CERT_FILE` and/or `SSL_CERT_DIR` environment variable is
/// configured in this process with an existing file or directory.
///
/// That being the case would indicate that certificates will be found successfully by OpenSSL.
///
/// Returns `true` if either variable is set to an existing file or directory.
pub fn has_ssl_cert_env_vars() -> bool {
    let probe = probe_from_env();
    probe.cert_file.is_some() || probe.cert_dir.is_some()
}
