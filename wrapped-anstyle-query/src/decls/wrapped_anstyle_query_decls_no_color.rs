use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Check [NO_COLOR] status
///
/// When `true`, should prevent the addition of ANSI color.
///
/// User-level configuration files and per-instance command-line arguments should override
/// [NO_COLOR]. A user should be able to export `$NO_COLOR` in their shell configuration file as a
/// default, but configure a specific program in its configuration file to specifically enable
/// color.
///
/// [NO_COLOR]: https://no-color.org/
#[inline]
pub fn no_color() -> bool {
    non_empty(std::env::var_os("NO_COLOR").as_deref())
}
