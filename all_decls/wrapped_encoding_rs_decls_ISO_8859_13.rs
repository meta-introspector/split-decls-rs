use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The ISO-8859-13 encoding.
///
/// This is the Baltic part of the ISO/IEC 8859 encoding family. This encoding
/// is also known as Latin 7.
///
/// [Index visualization](https://encoding.spec.whatwg.org/iso-8859-13.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-13-bmp.html)
///
/// This encoding matches the Windows code page 28603, except Windows decodes
/// unassigned code points to the Private Use Area of Unicode.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static ISO_8859_13: &'static Encoding = &ISO_8859_13_INIT;
