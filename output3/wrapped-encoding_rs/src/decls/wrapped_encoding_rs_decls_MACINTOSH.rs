use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The macintosh encoding.
///
/// This is the MacRoman encoding from Mac OS Classic.
///
/// [Index visualization](https://encoding.spec.whatwg.org/macintosh.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/macintosh-bmp.html)
///
/// This encoding matches the Windows code page 10000, except Windows decodes
/// 0xBD to U+2126 OHM SIGN instead of U+03A9 GREEK CAPITAL LETTER OMEGA.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static MACINTOSH: &'static Encoding = &MACINTOSH_INIT;
