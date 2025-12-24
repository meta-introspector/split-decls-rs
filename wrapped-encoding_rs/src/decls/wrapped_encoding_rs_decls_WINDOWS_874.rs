use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The windows-874 encoding.
///
/// This is the Thai encoding for Windows. It is an extension of TIS-620 / ISO-8859-11.
///
/// [Index visualization](https://encoding.spec.whatwg.org/windows-874.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-874-bmp.html)
///
/// This encoding matches the Windows code page 874, except Windows decodes
/// unassigned code points to the Private Use Area of Unicode.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static WINDOWS_874: &'static Encoding = &WINDOWS_874_INIT;
