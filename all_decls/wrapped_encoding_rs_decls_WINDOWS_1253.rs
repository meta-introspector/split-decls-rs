use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The windows-1253 encoding.
///
/// This is the Greek encoding for Windows. It is mostly an extension of
/// ISO-8859-7, but U+0386 is mapped to a different byte.
///
/// [Index visualization](https://encoding.spec.whatwg.org/windows-1253.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1253-bmp.html)
///
/// This encoding matches the Windows code page 1253, except Windows decodes
/// unassigned code points to the Private Use Area of Unicode.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static WINDOWS_1253: &'static Encoding = &WINDOWS_1253_INIT;
