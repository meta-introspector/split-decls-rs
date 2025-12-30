// Generated macro for macro_10104 (macro)
macro_rules! Depcrate_transmutemacro_10104 {
() => {
// Module: crate::transmute
// Provides: {"macro_10104"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for transmutes from a `&[u8]` to a `&str`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Not every byte slice is a valid UTF-8 string."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " - [`from_utf8`] which this lint suggests using is slower than `transmute`"] # [doc = " as it needs to validate the input."] # [doc = " If you are certain that the input is always a valid UTF-8,"] # [doc = " use [`from_utf8_unchecked`] which is as fast as `transmute`"] # [doc = " but has a semantically meaningful name."] # [doc = " - You might want to handle errors returned from [`from_utf8`] instead of calling `unwrap`."] # [doc = ""] # [doc = " [`from_utf8`]: https://doc.rust-lang.org/std/str/fn.from_utf8.html"] # [doc = " [`from_utf8_unchecked`]: https://doc.rust-lang.org/std/str/fn.from_utf8_unchecked.html"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let b: &[u8] = &[1_u8, 2_u8];"] # [doc = " unsafe {"] # [doc = "     let _: &str = std::mem::transmute(b); // where b: &[u8]"] # [doc = " }"] # [doc = ""] # [doc = " // should be:"] # [doc = " let _ = std::str::from_utf8(b).unwrap();"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub TRANSMUTE_BYTES_TO_STR , complexity , "transmutes from a `&[u8]` to a `&str`" }
};
}
