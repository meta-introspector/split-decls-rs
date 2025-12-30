// Generated macro for ISO_2022_JP (static)
macro_rules! DepcrateISO_2022_JP {
() => {
// Module: crate
// Provides: {"ISO_2022_JP"}
// Dependencies: {}
# [doc = " The ISO-2022-JP encoding."] # [doc = ""] # [doc = " This the primary pre-UTF-8 encoding for Japanese email. It uses the ASCII"] # [doc = " byte range to encode non-Basic Latin characters. It's the only encoding"] # [doc = " supported by this crate whose encoder is stateful."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/jis0208.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/jis0208-bmp.html)"] # [doc = ""] # [doc = " This encoding roughly matches the Windows code page 50220. Notably, Windows"] # [doc = " uses U+30FB in place of the REPLACEMENT CHARACTER and otherwise differs in"] # [doc = " error handling."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static ISO_2022_JP : & 'static Encoding = & ISO_2022_JP_INIT ;
};
}
