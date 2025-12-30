// Generated macro for enc_csel (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_csel {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_csel"}
// Dependencies: {}
fn enc_csel (rd : Writable < Reg > , rn : Reg , rm : Reg , cond : Cond , op : u32 , o2 : u32) -> u32 { debug_assert_eq ! (op & 0b1 , op) ; debug_assert_eq ! (o2 & 0b1 , o2) ; 0b100_11010100_00000_0000_00_00000_00000 | (op << 30) | (machreg_to_gpr (rm) << 16) | (cond . bits () << 12) | (o2 << 10) | (machreg_to_gpr (rn) << 5) | machreg_to_gpr (rd . to_reg ()) }
};
}
