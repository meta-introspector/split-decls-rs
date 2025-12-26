use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The x-user-defined encoding.
///
/// This encoding offsets the non-ASCII bytes by `0xF700` thereby decoding
/// them to the Private Use Area of Unicode. It was used for loading binary
/// data into a JavaScript string using `XMLHttpRequest` before XHR supported
/// the `"arraybuffer"` response type.
///
/// This encoding does not have a Windows code page number.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static X_USER_DEFINED: &'static Encoding = &X_USER_DEFINED_INIT;
