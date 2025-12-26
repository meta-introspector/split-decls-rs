use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(target_os = "unknown")]
impl IsTerminal for std::process::ChildStderr {
    #[inline]
    fn is_terminal(&self) -> bool {
        false
    }
}
