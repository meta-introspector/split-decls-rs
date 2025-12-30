// Generated macro for impl_71 (impl)
macro_rules! Depcrate_parser_sliceimpl_71 {
() => {
// Module: crate::parser::slice
// Provides: {"impl_71"}
// Dependencies: {}
impl Slice < '_ > for String { fn slice (& self , range : Range < usize >) -> Self { self [range] . to_string () } fn trim (& mut self) { * self = self . trim_end_matches (matches_fluent_ws) . to_string () ; } }
};
}
