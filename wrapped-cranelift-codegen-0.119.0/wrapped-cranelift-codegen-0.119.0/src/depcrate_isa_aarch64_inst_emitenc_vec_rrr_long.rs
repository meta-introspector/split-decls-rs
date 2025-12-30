// Generated macro for enc_vec_rrr_long (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_vec_rrr_long {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_vec_rrr_long"}
// Dependencies: {}
fn enc_vec_rrr_long (q : u32 , u : u32 , size : u32 , bit14 : u32 , rm : Reg , rn : Reg , rd : Writable < Reg > ,) -> u32 { debug_assert_eq ! (q & 0b1 , q) ; debug_assert_eq ! (u & 0b1 , u) ; debug_assert_eq ! (size & 0b11 , size) ; debug_assert_eq ! (bit14 & 0b1 , bit14) ; 0b0_0_0_01110_00_1_00000_100000_00000_00000 | q << 30 | u << 29 | size << 22 | bit14 << 14 | (machreg_to_vec (rm) << 16) | (machreg_to_vec (rn) << 5) | machreg_to_vec (rd . to_reg ()) }
};
}
