use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The UTF-16BE encoding.
///
/// This decode-only encoding uses 16-bit code units due to Unicode originally
/// having been designed as a 16-bit reportoire. In the absence of a byte order
/// mark the big endian byte order is assumed.
///
/// There is no corresponding encoder in this crate or in the Encoding
/// Standard. The output encoding of this encoding is UTF-8.
///
/// This encoding matches the Windows code page 1201.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static UTF_16BE: &'static Encoding = &UTF_16BE_INIT;
