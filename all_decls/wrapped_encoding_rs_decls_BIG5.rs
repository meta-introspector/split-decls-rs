use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The Big5 encoding.
///
/// This is Big5 with HKSCS with mappings to more recent Unicode assignments
/// instead of the Private Use Area code points that have been used historically.
/// It is believed to be able to decode existing Web content in a way that makes
/// sense.
///
/// To avoid form submissions generating data that Web servers don't understand,
/// the encoder doesn't use the HKSCS byte sequences that precede the unextended
/// Big5 in the lexical order.
///
/// [Index visualization](https://encoding.spec.whatwg.org/big5.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/big5-bmp.html)
///
/// This encoding is designed to be suited for decoding the Windows code page 950
/// and its HKSCS patched "951" variant such that the text makes sense, given
/// assignments that Unicode has made after those encodings used Private Use
/// Area characters.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static BIG5: &'static Encoding = &BIG5_INIT;
