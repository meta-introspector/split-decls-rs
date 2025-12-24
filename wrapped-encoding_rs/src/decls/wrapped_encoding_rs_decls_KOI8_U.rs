use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The KOI8-U encoding.
///
/// This is an encoding for Ukrainian adapted from KOI8-R.
///
/// [Index visualization](https://encoding.spec.whatwg.org/koi8-u.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/koi8-u-bmp.html)
///
/// This encoding matches the Windows code page 21866.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static KOI8_U: &'static Encoding = &KOI8_U_INIT;
