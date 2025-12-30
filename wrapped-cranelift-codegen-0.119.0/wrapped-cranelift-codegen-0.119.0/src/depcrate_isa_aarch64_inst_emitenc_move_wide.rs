// Generated macro for enc_move_wide (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_move_wide {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_move_wide"}
// Dependencies: {}
fn enc_move_wide (op : MoveWideOp , rd : Writable < Reg > , imm : MoveWideConst , size : OperandSize) -> u32 { assert ! (imm . shift <= 0b11) ; let op = match op { MoveWideOp :: MovN => 0b00 , MoveWideOp :: MovZ => 0b10 , } ; 0x12800000 | size . sf_bit () << 31 | op << 29 | u32 :: from (imm . shift) << 21 | u32 :: from (imm . bits) << 5 | machreg_to_gpr (rd . to_reg ()) }
};
}
