// Generated macro for enc_test_bit_and_branch (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_test_bit_and_branch {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_test_bit_and_branch"}
// Dependencies: {}
fn enc_test_bit_and_branch (kind : TestBitAndBranchKind , taken : BranchTarget , reg : Reg , bit : u8 ,) -> u32 { assert ! (bit < 64) ; let op_31 = u32 :: from (bit >> 5) ; let op_23_19 = u32 :: from (bit & 0b11111) ; let op_30_24 = 0b0110110 | match kind { TestBitAndBranchKind :: Z => 0 , TestBitAndBranchKind :: NZ => 1 , } ; (op_31 << 31) | (op_30_24 << 24) | (op_23_19 << 19) | (taken . as_offset14_or_zero () << 5) | machreg_to_gpr (reg) }
};
}
