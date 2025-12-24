use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Check [CLICOLOR_FORCE] status
///
/// ANSI colors should be enabled no matter what.
///
/// [CLICOLOR_FORCE]: https://bixense.com/clicolors/
#[inline]
pub fn clicolor_force() -> bool {
    non_empty(std::env::var_os("CLICOLOR_FORCE").as_deref())
}
