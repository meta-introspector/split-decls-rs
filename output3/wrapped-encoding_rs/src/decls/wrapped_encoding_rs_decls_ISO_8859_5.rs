use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The ISO-8859-5 encoding.
///
/// This is the Cyrillic part of the ISO/IEC 8859 encoding family.
///
/// [Index visualization](https://encoding.spec.whatwg.org/iso-8859-5.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-5-bmp.html)
///
/// This encoding matches the Windows code page 28595.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static ISO_8859_5: &'static Encoding = &ISO_8859_5_INIT;
