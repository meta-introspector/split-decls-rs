// Generated macro for impl_123 (impl)
macro_rules! Depcrateimpl_123 {
() => {
// Module: crate
// Provides: {"impl_123"}
// Dependencies: {}
impl < 'a > Iterator for Iter < 'a > { type Item = & 'a str ; # [inline] fn next (& mut self) -> Option < & 'a str > { self . inner . next () . map (Component :: as_str) } }
};
}
