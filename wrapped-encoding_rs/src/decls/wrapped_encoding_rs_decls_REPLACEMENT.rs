use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The replacement encoding.
///
/// This decode-only encoding decodes all non-zero-length streams to a single
/// REPLACEMENT CHARACTER. Its purpose is to avoid the use of an
/// ASCII-compatible fallback encoding (typically windows-1252) for some
/// encodings that are no longer supported by the Web Platform and that
/// would be dangerous to treat as ASCII-compatible.
///
/// There is no corresponding encoder. The output encoding of this encoding
/// is UTF-8.
///
/// This encoding does not have a Windows code page number.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static REPLACEMENT: &'static Encoding = &REPLACEMENT_INIT;
