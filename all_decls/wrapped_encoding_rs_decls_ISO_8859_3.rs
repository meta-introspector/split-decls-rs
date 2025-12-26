use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The ISO-8859-3 encoding.
///
/// This is the South European part of the ISO/IEC 8859 encoding family. This encoding is also known as Latin 3.
///
/// [Index visualization](https://encoding.spec.whatwg.org/iso-8859-3.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-3-bmp.html)
///
/// This encoding matches the Windows code page 28593.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static ISO_8859_3: &'static Encoding = &ISO_8859_3_INIT;
