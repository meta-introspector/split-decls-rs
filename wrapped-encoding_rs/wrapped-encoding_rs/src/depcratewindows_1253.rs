// Generated macro for WINDOWS_1253 (static)
macro_rules! DepcrateWINDOWS_1253 {
() => {
// Module: crate
// Provides: {"WINDOWS_1253"}
// Dependencies: {}
# [doc = " The windows-1253 encoding."] # [doc = ""] # [doc = " This is the Greek encoding for Windows. It is mostly an extension of"] # [doc = " ISO-8859-7, but U+0386 is mapped to a different byte."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/windows-1253.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1253-bmp.html)"] # [doc = ""] # [doc = " This encoding matches the Windows code page 1253, except Windows decodes"] # [doc = " unassigned code points to the Private Use Area of Unicode."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static WINDOWS_1253 : & 'static Encoding = & WINDOWS_1253_INIT ;
};
}
