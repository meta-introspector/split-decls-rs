use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Run `$bin` in the test's environment, see [`ProcessBuilder`]
///
/// For more on the test environment, see
/// - [`paths::root`]
/// - [`TestEnvCommandExt`]
pub fn process<T: AsRef<OsStr>>(bin: T) -> ProcessBuilder {
    _process(bin.as_ref())
}
