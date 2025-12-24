use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The windows-1257 encoding.
///
/// This is the Baltic encoding for Windows.
///
/// [Index visualization](https://encoding.spec.whatwg.org/windows-1257.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1257-bmp.html)
///
/// This encoding matches the Windows code page 1257, except Windows decodes
/// unassigned code points to the Private Use Area of Unicode.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static WINDOWS_1257: &'static Encoding = &WINDOWS_1257_INIT;
