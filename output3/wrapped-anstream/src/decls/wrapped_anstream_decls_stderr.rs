use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Create an ANSI escape code compatible stderr
///
/// **Note:** Call [`AutoStream::lock`] in loops to avoid the performance hit of acquiring/releasing
/// from the implicit locking in each [`std::io::Write`] call
#[cfg(feature = "auto")]
pub fn stderr() -> Stderr {
    let stderr = std::io::stderr();
    AutoStream::auto(stderr)
}
