use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Check `TERM` for ANSI color support
///
/// On Windows, you might need to also check [`windows::enable_ansi_colors`] as ANSI color support
/// is opt-in, rather than assumed.
#[inline]
pub fn term_supports_ansi_color() -> bool {
    #[cfg(not(windows))]
    {
        term_supports_color()
    }
    #[cfg(windows)]
    {
        match std::env::var_os("TERM") {
            None => return false,
            Some(k) => {
                if k == "dumb" || k == "cygwin" {
                    return false;
                }
            }
        }
        true
    }
}
