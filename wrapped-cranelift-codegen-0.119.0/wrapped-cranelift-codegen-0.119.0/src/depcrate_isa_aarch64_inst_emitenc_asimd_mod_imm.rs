// Generated macro for enc_asimd_mod_imm (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_asimd_mod_imm {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_asimd_mod_imm"}
// Dependencies: {}
fn enc_asimd_mod_imm (rd : Writable < Reg > , q_op : u32 , cmode : u32 , imm : u8) -> u32 { let abc = (imm >> 5) as u32 ; let defgh = (imm & 0b11111) as u32 ; debug_assert_eq ! (cmode & 0b1111 , cmode) ; debug_assert_eq ! (q_op & 0b11 , q_op) ; 0b0_0_0_0111100000_000_0000_01_00000_00000 | (q_op << 29) | (abc << 16) | (cmode << 12) | (defgh << 5) | machreg_to_vec (rd . to_reg ()) }
};
}
