use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Check [CLICOLOR] status
///
/// - When `true`, ANSI colors are supported and should be used when the program isn't piped,
///   similar to [`term_supports_color`]
/// - When `false`, don’t output ANSI color escape codes, similar to [`no_color`]
///
/// See also:
/// - [terminfo](https://crates.io/crates/terminfo) or [term](https://crates.io/crates/term) for
///   checking termcaps
/// - [termbg](https://crates.io/crates/termbg) for detecting background color
///
/// [CLICOLOR]: https://bixense.com/clicolors/
#[inline]
pub fn clicolor() -> Option<bool> {
    let value = std::env::var_os("CLICOLOR")?;
    Some(value != "0")
}
