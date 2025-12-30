// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
impl DoubleEndedIterator for Utf8Components < '_ > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { self . 0 . next_back () . map (| component | { unsafe { Utf8Component :: new (component) } }) } }
};
}
