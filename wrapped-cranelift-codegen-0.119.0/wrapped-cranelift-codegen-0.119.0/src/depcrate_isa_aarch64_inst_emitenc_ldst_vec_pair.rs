// Generated macro for enc_ldst_vec_pair (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_ldst_vec_pair {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_ldst_vec_pair"}
// Dependencies: {}
fn enc_ldst_vec_pair (opc : u32 , amode : u32 , is_load : bool , simm7 : SImm7Scaled , rn : Reg , rt : Reg , rt2 : Reg ,) -> u32 { debug_assert_eq ! (opc & 0b11 , opc) ; debug_assert_eq ! (amode & 0b11 , amode) ; 0b00_10110_00_0_0000000_00000_00000_00000 | opc << 30 | amode << 23 | (is_load as u32) << 22 | simm7 . bits () << 15 | machreg_to_vec (rt2) << 10 | machreg_to_gpr (rn) << 5 | machreg_to_vec (rt) }
};
}
