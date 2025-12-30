// Generated macro for enc_conditional_br (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_conditional_br {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_conditional_br"}
// Dependencies: {}
fn enc_conditional_br (taken : BranchTarget , kind : CondBrKind) -> u32 { match kind { CondBrKind :: Zero (reg , size) => enc_op_size (enc_cmpbr (0b0_011010_0 , taken . as_offset19_or_zero () , reg) , size ,) , CondBrKind :: NotZero (reg , size) => enc_op_size (enc_cmpbr (0b0_011010_1 , taken . as_offset19_or_zero () , reg) , size ,) , CondBrKind :: Cond (c) => enc_cbr (0b01010100 , taken . as_offset19_or_zero () , 0b0 , c . bits ()) , } }
};
}
