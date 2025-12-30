// Generated macro for impl_1708 (impl)
macro_rules! Depcrate_isa_aarch64_inst_immsimpl_1708 {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"impl_1708"}
// Dependencies: {}
impl PrettyPrint for Imm12 { fn pretty_print (& self , _ : u8) -> String { let shift = if self . shift12 { 12 } else { 0 } ; let value = u32 :: from (self . bits) << shift ; format ! ("#{value}") } }
};
}
