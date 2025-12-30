// Generated macro for reg_to_gpr_num (function)
macro_rules! Depcrate_isa_riscv64_inst_emitreg_to_gpr_num {
() => {
// Module: crate::isa::riscv64::inst::emit
// Provides: {"reg_to_gpr_num"}
// Dependencies: {}
pub (crate) fn reg_to_gpr_num (m : Reg) -> u32 { u32 :: from (m . to_real_reg () . unwrap () . hw_enc () & 31) }
};
}
