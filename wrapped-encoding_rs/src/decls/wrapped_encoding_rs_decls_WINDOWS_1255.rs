use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The windows-1255 encoding.
///
/// This is the Hebrew encoding for Windows. It is an extension of ISO-8859-8-I,
/// except for a currency sign swap.
///
/// [Index visualization](https://encoding.spec.whatwg.org/windows-1255.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1255-bmp.html)
///
/// This encoding matches the Windows code page 1255, except Windows decodes
/// unassigned code points to the Private Use Area of Unicode.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static WINDOWS_1255: &'static Encoding = &WINDOWS_1255_INIT;
