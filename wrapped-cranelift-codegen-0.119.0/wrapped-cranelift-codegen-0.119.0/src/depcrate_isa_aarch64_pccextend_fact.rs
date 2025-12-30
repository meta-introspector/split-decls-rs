// Generated macro for extend_fact (function)
macro_rules! Depcrate_isa_aarch64_pccextend_fact {
() => {
// Module: crate::isa::aarch64::pcc
// Provides: {"extend_fact"}
// Dependencies: {}
fn extend_fact (ctx : & FactContext , value : & Fact , mode : ExtendOp) -> Option < Fact > { match mode { ExtendOp :: UXTB => ctx . uextend (value , 8 , 64) , ExtendOp :: UXTH => ctx . uextend (value , 16 , 64) , ExtendOp :: UXTW => ctx . uextend (value , 32 , 64) , ExtendOp :: UXTX => Some (value . clone ()) , ExtendOp :: SXTB => ctx . sextend (value , 8 , 64) , ExtendOp :: SXTH => ctx . sextend (value , 16 , 64) , ExtendOp :: SXTW => ctx . sextend (value , 32 , 64) , ExtendOp :: SXTX => None , } }
};
}
