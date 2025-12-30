// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
impl < 'a > Iterator for Shlex < 'a > { type Item = String ; fn next (& mut self) -> Option < String > { self . 0 . next () . map (| byte_word | { unsafe { String :: from_utf8_unchecked (byte_word) } }) } }
};
}
