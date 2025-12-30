// Generated macro for impl_813 (impl)
macro_rules! Depcrate_ir_immediatesimpl_813 {
() => {
// Module: crate::ir::immediates
// Provides: {"impl_813"}
// Dependencies: {}
impl FromStr for Uimm64 { type Err = & 'static str ; fn from_str (s : & str) -> Result < Self , & 'static str > { parse_u64 (s) . map (Self :: new) } }
};
}
