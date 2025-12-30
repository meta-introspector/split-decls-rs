// Generated macro for enc_vec_rr_pair_long (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_vec_rr_pair_long {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_vec_rr_pair_long"}
// Dependencies: {}
fn enc_vec_rr_pair_long (u : u32 , enc_size : u32 , rd : Writable < Reg > , rn : Reg) -> u32 { debug_assert_eq ! (u & 0b1 , u) ; debug_assert_eq ! (enc_size & 0b1 , enc_size) ; 0b0_1_0_01110_00_10000_00_0_10_10_00000_00000 | u << 29 | enc_size << 22 | machreg_to_vec (rn) << 5 | machreg_to_vec (rd . to_reg ()) }
};
}
