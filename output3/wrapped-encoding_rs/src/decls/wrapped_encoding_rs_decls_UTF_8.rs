use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The UTF-8 encoding.
///
/// This is the encoding that should be used for all new development it can
/// represent all of Unicode.
///
/// This encoding matches the Windows code page 65001, except Windows differs
/// in the number of errors generated for some erroneous byte sequences.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static UTF_8: &'static Encoding = &UTF_8_INIT;
