// Generated macro for impl_123 (impl)
macro_rules! Depcrate_parse_typesimpl_123 {
() => {
// Module: crate::parse::types
// Provides: {"impl_123"}
// Dependencies: {}
impl < 'a > ErrorDetail < 'a > { pub fn new (input : & 'a str , message : impl Into < String >) -> Self { let input = & input [.. input . len () . min (1)] ; Self { input , message : message . into () } } }
};
}
