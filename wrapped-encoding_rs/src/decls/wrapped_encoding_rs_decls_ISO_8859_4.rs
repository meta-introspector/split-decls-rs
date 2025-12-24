use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The ISO-8859-4 encoding.
///
/// This is the North European part of the ISO/IEC 8859 encoding family. This encoding is also known as Latin 4.
///
/// [Index visualization](https://encoding.spec.whatwg.org/iso-8859-4.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-4-bmp.html)
///
/// This encoding matches the Windows code page 28594.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static ISO_8859_4: &'static Encoding = &ISO_8859_4_INIT;
