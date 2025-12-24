use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The windows-1254 encoding.
///
/// This is the Turkish encoding for Windows. It is an extension of ISO-8859-9,
/// which is known as Latin 5.
///
/// [Index visualization](https://encoding.spec.whatwg.org/windows-1254.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1254-bmp.html)
///
/// This encoding matches the Windows code page 1254.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static WINDOWS_1254: &'static Encoding = &WINDOWS_1254_INIT;
