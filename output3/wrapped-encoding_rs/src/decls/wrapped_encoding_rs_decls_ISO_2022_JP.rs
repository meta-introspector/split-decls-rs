use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The ISO-2022-JP encoding.
///
/// This the primary pre-UTF-8 encoding for Japanese email. It uses the ASCII
/// byte range to encode non-Basic Latin characters. It's the only encoding
/// supported by this crate whose encoder is stateful.
///
/// [Index visualization](https://encoding.spec.whatwg.org/jis0208.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/jis0208-bmp.html)
///
/// This encoding roughly matches the Windows code page 50220. Notably, Windows
/// uses U+30FB in place of the REPLACEMENT CHARACTER and otherwise differs in
/// error handling.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static ISO_2022_JP: &'static Encoding = &ISO_2022_JP_INIT;
