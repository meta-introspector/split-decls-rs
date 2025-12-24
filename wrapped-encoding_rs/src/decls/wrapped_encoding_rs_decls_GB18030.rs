use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The gb18030 encoding.
///
/// This encoding matches GB18030-2022 except the two-byte sequence 0xA3 0xA0
/// maps to U+3000 for compatibility with existing Web content and the four-byte
/// sequences for the non-PUA characters that got two-byte sequences still decode
/// to the same non-PUA characters as in GB18030-2005. As a result, this encoding
/// can represent all of Unicode except for 19 private-use characters.
///
/// [Index visualization for the two-byte sequences](https://encoding.spec.whatwg.org/gb18030.html),
/// [Visualization of BMP coverage of the two-byte index](https://encoding.spec.whatwg.org/gb18030-bmp.html)
///
/// This encoding matches the Windows code page 54936.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static GB18030: &'static Encoding = &GB18030_INIT;
