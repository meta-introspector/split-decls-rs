// Generated macro for reg_to_compressed_gpr_num (function)
macro_rules! Depcrate_isa_riscv64_inst_emitreg_to_compressed_gpr_num {
() => {
// Module: crate::isa::riscv64::inst::emit
// Provides: {"reg_to_compressed_gpr_num"}
// Dependencies: {}
pub (crate) fn reg_to_compressed_gpr_num (m : Reg) -> u32 { let real_reg = m . to_real_reg () . unwrap () . hw_enc () ; debug_assert ! (real_reg >= 8 && real_reg < 16) ; let compressed_reg = real_reg - 8 ; u32 :: from (compressed_reg) }
};
}
