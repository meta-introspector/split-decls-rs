// Generated macro for impl_62 (impl)
macro_rules! Depcrateimpl_62 {
() => {
// Module: crate
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'a > Iterator for Iter < 'a > { type Item = & 'a str ; # [inline] fn next (& mut self) -> Option < & 'a str > { self . inner . next () . map (| component | component . as_str ()) } }
};
}
