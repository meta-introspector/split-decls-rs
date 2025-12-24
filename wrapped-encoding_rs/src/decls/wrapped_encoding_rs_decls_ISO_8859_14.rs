use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The ISO-8859-14 encoding.
///
/// This is the Celtic part of the ISO/IEC 8859 encoding family. This encoding
/// is also known as Latin 8.
///
/// [Index visualization](https://encoding.spec.whatwg.org/iso-8859-14.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-14-bmp.html)
///
/// The Windows code page number for this encoding is 28604, but kernel32.dll
/// does not support this encoding.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static ISO_8859_14: &'static Encoding = &ISO_8859_14_INIT;
