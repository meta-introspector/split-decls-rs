// Generated macro for impl_267 (impl)
macro_rules! Depcrate_string_arrayimpl_267 {
() => {
// Module: crate::string_array
// Provides: {"impl_267"}
// Dependencies: {}
impl < 'a > Iterator for Iter < 'a > { type Item = Option < & 'a str > ; fn next (& mut self) -> Option < Option < & 'a str > > { self . range . next () . map (| i | self . arr . get (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
};
}
