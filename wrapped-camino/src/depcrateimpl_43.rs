// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
impl < 'a > Iterator for Utf8Ancestors < 'a > { type Item = & 'a Utf8Path ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () . map (| path | { unsafe { Utf8Path :: assume_utf8 (path) } }) } }
};
}
