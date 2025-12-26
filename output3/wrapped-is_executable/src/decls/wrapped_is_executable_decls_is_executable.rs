use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns `true` if there is a file at the given path and it is
/// executable. Returns `false` otherwise.
///
/// See the module documentation for details.
pub fn is_executable<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    path.as_ref().is_executable()
}
