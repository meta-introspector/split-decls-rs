// Generated macro for impl_124 (impl)
macro_rules! Depcrateimpl_124 {
() => {
// Module: crate
// Provides: {"impl_124"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for Iter < 'a > { # [inline] fn next_back (& mut self) -> Option < & 'a str > { self . inner . next_back () . map (Component :: as_str) } }
};
}
