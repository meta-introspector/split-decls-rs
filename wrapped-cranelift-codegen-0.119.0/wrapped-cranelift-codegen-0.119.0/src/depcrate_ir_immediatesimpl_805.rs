// Generated macro for impl_805 (impl)
macro_rules! Depcrate_ir_immediatesimpl_805 {
() => {
// Module: crate::ir::immediates
// Provides: {"impl_805"}
// Dependencies: {}
impl FromStr for Imm64 { type Err = & 'static str ; fn from_str (s : & str) -> Result < Self , & 'static str > { parse_i64 (s) . map (Self :: new) } }
};
}
