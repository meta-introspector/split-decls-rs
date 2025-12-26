use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Probe the system for the directory in which CA certificates should likely be
/// found.
///
/// This will only search known system locations.
#[doc(hidden)]
#[deprecated(note = "use `candidate_cert_dirs` instead")]
pub fn find_certs_dirs() -> Vec<PathBuf> {
    candidate_cert_dirs().map(Path::to_path_buf).collect()
}
