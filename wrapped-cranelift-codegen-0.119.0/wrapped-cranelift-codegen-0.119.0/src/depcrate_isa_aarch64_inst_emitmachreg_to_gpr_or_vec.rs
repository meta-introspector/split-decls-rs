// Generated macro for machreg_to_gpr_or_vec (function)
macro_rules! Depcrate_isa_aarch64_inst_emitmachreg_to_gpr_or_vec {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"machreg_to_gpr_or_vec"}
// Dependencies: {}
fn machreg_to_gpr_or_vec (m : Reg) -> u32 { u32 :: from (m . to_real_reg () . unwrap () . hw_enc () & 31) }
};
}
