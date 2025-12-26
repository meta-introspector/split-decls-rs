use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The ISO-8859-8 encoding.
///
/// This is the Hebrew part of the ISO/IEC 8859 encoding family in visual order.
///
/// [Index visualization](https://encoding.spec.whatwg.org/iso-8859-8.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-8-bmp.html)
///
/// This encoding roughly matches the Windows code page 28598. Windows decodes
/// 0xAF to OVERLINE instead of MACRON and 0xFE and 0xFD to the Private Use
/// Area instead of LRM and RLM. Windows decodes unassigned code points to
/// the private use area.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static ISO_8859_8: &'static Encoding = &ISO_8859_8_INIT;
