// Generated macro for impl_72 (impl)
macro_rules! Depcrate_parser_sliceimpl_72 {
() => {
// Module: crate::parser::slice
// Provides: {"impl_72"}
// Dependencies: {}
impl < 's > Slice < 's > for & 's str { fn slice (& self , range : Range < usize >) -> Self { & self [range] } fn trim (& mut self) { * self = self . trim_end_matches (matches_fluent_ws) ; } }
};
}
