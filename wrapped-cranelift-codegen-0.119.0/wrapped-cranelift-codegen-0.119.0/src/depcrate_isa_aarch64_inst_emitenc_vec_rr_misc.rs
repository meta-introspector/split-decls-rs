// Generated macro for enc_vec_rr_misc (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_vec_rr_misc {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_vec_rr_misc"}
// Dependencies: {}
fn enc_vec_rr_misc (qu : u32 , size : u32 , bits_12_16 : u32 , rd : Writable < Reg > , rn : Reg) -> u32 { debug_assert_eq ! (qu & 0b11 , qu) ; debug_assert_eq ! (size & 0b11 , size) ; debug_assert_eq ! (bits_12_16 & 0b11111 , bits_12_16) ; let bits = 0b0_00_01110_00_10000_00000_10_00000_00000 ; bits | qu << 29 | size << 22 | bits_12_16 << 12 | machreg_to_vec (rn) << 5 | machreg_to_vec (rd . to_reg ()) }
};
}
