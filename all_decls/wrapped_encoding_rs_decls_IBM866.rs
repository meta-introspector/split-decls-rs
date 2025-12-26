use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The IBM866 encoding.
///
/// This the most notable one of the DOS Cyrillic code pages. It has the same
/// box drawing characters as code page 437, so it can be used for decoding
/// DOS-era ASCII + box drawing data.
///
/// [Index visualization](https://encoding.spec.whatwg.org/ibm866.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/ibm866-bmp.html)
///
/// This encoding matches the Windows code page 866.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static IBM866: &'static Encoding = &IBM866_INIT;
