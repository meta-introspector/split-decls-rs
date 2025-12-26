use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The windows-1252 encoding.
///
/// This is the Western encoding for Windows. It is an extension of ISO-8859-1,
/// which is known as Latin 1.
///
/// [Index visualization](https://encoding.spec.whatwg.org/windows-1252.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1252-bmp.html)
///
/// This encoding matches the Windows code page 1252.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static WINDOWS_1252: &'static Encoding = &WINDOWS_1252_INIT;
