use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The EUC-KR encoding.
///
/// This is the Korean encoding for Windows. It extends the Unix legacy encoding
/// for Korean, based on KS X 1001 (which also formed the base of MacKorean on Mac OS
/// Classic), with all the characters from the Hangul Syllables block of Unicode.
///
/// [Index visualization](https://encoding.spec.whatwg.org/euc-kr.html),
/// [Visualization of BMP coverage](https://encoding.spec.whatwg.org/euc-kr-bmp.html)
///
/// This encoding matches the Windows code page 949, except Windows decodes byte 0x80
/// to U+0080 and some byte sequences that are error per the Encoding Standard to
/// the question mark or the Private Use Area.
///
/// This will change from `static` to `const` if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate, so don't take the address of this
/// `static`.
pub static EUC_KR: &'static Encoding = &EUC_KR_INIT;
