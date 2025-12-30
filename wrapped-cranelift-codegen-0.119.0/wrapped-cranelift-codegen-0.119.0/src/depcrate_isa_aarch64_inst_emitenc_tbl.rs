// Generated macro for enc_tbl (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_tbl {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_tbl"}
// Dependencies: {}
fn enc_tbl (is_extension : bool , len : u32 , rd : Writable < Reg > , rn : Reg , rm : Reg) -> u32 { debug_assert_eq ! (len & 0b11 , len) ; 0b0_1_001110_000_00000_0_00_0_00_00000_00000 | (machreg_to_vec (rm) << 16) | len << 13 | (is_extension as u32) << 12 | (machreg_to_vec (rn) << 5) | machreg_to_vec (rd . to_reg ()) }
};
}
