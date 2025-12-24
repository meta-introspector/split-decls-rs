use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The OpenSSL environment variable to configure what certificates directory to use.
pub const ENV_CERT_DIR: &'static str = "SSL_CERT_DIR";
