use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(all(not(miri), unix, any(target_env = "gnu", target_os = "macos"))))]
mod signal_handler {
    /// On platforms which don't support our signal handler's requirements,
    /// simply use the default signal handler provided by std.
    pub(super) fn install() {}
}
