// Generated macro for machreg_to_gpr (function)
macro_rules! Depcrate_isa_aarch64_inst_emitmachreg_to_gpr {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"machreg_to_gpr"}
// Dependencies: {}
pub (crate) fn machreg_to_gpr (m : Reg) -> u32 { assert_eq ! (m . class () , RegClass :: Int) ; u32 :: from (m . to_real_reg () . unwrap () . hw_enc () & 31) }
};
}
