// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for Iter < 'a > { # [inline] fn next_back (& mut self) -> Option < & 'a str > { self . inner . next_back () . map (| component | component . as_str ()) } }
};
}
