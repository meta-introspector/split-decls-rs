// Generated macro for impl_47 (impl)
macro_rules! Depcrateimpl_47 {
() => {
// Module: crate
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'a > Iterator for Utf8Components < 'a > { type Item = Utf8Component < 'a > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () . map (| component | { unsafe { Utf8Component :: new (component) } }) } }
};
}
