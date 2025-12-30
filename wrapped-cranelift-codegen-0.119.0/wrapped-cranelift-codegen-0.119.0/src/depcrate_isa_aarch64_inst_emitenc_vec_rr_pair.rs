// Generated macro for enc_vec_rr_pair (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_vec_rr_pair {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_vec_rr_pair"}
// Dependencies: {}
fn enc_vec_rr_pair (bits_12_16 : u32 , rd : Writable < Reg > , rn : Reg) -> u32 { debug_assert_eq ! (bits_12_16 & 0b11111 , bits_12_16) ; 0b010_11110_11_11000_11011_10_00000_00000 | bits_12_16 << 12 | machreg_to_vec (rn) << 5 | machreg_to_vec (rd . to_reg ()) }
};
}
