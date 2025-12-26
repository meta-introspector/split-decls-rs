use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The KOI8-R encoding.
///
/// This is an encoding for Russian from [RFC 1489](https://tools.ietf.org/html/rfc1489).
///
/// [Index visualization](https://encoding.spec.whatwg.org/koi8-r.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/koi8-r-bmp.html)
///
/// This encoding matches the Windows code page 20866.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static KOI8_R: &'static Encoding = &KOI8_R_INIT;
