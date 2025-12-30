// Generated macro for enc_ldst_vec (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_ldst_vec {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_ldst_vec"}
// Dependencies: {}
fn enc_ldst_vec (q : u32 , size : u32 , rn : Reg , rt : Writable < Reg >) -> u32 { debug_assert_eq ! (q & 0b1 , q) ; debug_assert_eq ! (size & 0b11 , size) ; 0b0_0_0011010_10_00000_110_0_00_00000_00000 | q << 30 | size << 10 | machreg_to_gpr (rn) << 5 | machreg_to_vec (rt . to_reg ()) }
};
}
