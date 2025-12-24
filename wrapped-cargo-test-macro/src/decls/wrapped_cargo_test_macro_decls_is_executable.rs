use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(windows)]
fn is_executable<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_file()
}
