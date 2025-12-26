use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The x-mac-cyrillic encoding.
///
/// This is the MacUkrainian encoding from Mac OS Classic.
///
/// [Index visualization](https://encoding.spec.whatwg.org/x-mac-cyrillic.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/x-mac-cyrillic-bmp.html)
///
/// This encoding matches the Windows code page 10017.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static X_MAC_CYRILLIC: &'static Encoding = &X_MAC_CYRILLIC_INIT;
