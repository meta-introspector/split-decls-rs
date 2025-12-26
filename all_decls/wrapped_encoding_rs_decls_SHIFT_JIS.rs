use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The Shift_JIS encoding.
///
/// This is the Japanese encoding for Windows.
///
/// [Index visualization](https://encoding.spec.whatwg.org/shift_jis.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/shift_jis-bmp.html)
///
/// This encoding matches the Windows code page 932, except Windows decodes some byte
/// sequences that are error per the Encoding Standard to the question mark or the
/// Private Use Area and generally uses U+30FB in place of the REPLACEMENT CHARACTER.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static SHIFT_JIS: &'static Encoding = &SHIFT_JIS_INIT;
