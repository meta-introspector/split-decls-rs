// Generated macro for machreg_to_vec (function)
macro_rules! Depcrate_isa_aarch64_inst_emitmachreg_to_vec {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"machreg_to_vec"}
// Dependencies: {}
pub (crate) fn machreg_to_vec (m : Reg) -> u32 { assert_eq ! (m . class () , RegClass :: Float) ; u32 :: from (m . to_real_reg () . unwrap () . hw_enc ()) }
};
}
