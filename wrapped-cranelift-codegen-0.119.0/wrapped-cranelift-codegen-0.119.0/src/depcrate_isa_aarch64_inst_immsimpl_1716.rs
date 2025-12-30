// Generated macro for impl_1716 (impl)
macro_rules! Depcrate_isa_aarch64_inst_immsimpl_1716 {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"impl_1716"}
// Dependencies: {}
impl PrettyPrint for MoveWideConst { fn pretty_print (& self , _ : u8) -> String { if self . shift == 0 { format ! ("#{}" , self . bits) } else { format ! ("#{}, LSL #{}" , self . bits , self . shift * 16) } } }
};
}
