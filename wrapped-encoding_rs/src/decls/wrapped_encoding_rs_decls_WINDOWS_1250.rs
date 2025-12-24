use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The windows-1250 encoding.
///
/// This is the Central European encoding for Windows.
///
/// [Index visualization](https://encoding.spec.whatwg.org/windows-1250.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1250-bmp.html)
///
/// This encoding matches the Windows code page 1250.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static WINDOWS_1250: &'static Encoding = &WINDOWS_1250_INIT;
