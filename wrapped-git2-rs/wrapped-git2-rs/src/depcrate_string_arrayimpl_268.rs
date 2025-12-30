// Generated macro for impl_268 (impl)
macro_rules! Depcrate_string_arrayimpl_268 {
() => {
// Module: crate::string_array
// Provides: {"impl_268"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for Iter < 'a > { fn next_back (& mut self) -> Option < Option < & 'a str > > { self . range . next_back () . map (| i | self . arr . get (i)) } }
};
}
