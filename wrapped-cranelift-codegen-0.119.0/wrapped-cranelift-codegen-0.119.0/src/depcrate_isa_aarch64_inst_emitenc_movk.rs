// Generated macro for enc_movk (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_movk {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_movk"}
// Dependencies: {}
fn enc_movk (rd : Writable < Reg > , imm : MoveWideConst , size : OperandSize) -> u32 { assert ! (imm . shift <= 0b11) ; 0x72800000 | size . sf_bit () << 31 | u32 :: from (imm . shift) << 21 | u32 :: from (imm . bits) << 5 | machreg_to_gpr (rd . to_reg ()) }
};
}
