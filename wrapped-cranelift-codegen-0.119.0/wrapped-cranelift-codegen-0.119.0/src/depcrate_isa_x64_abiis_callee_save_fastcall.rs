// Generated macro for is_callee_save_fastcall (function)
macro_rules! Depcrate_isa_x64_abiis_callee_save_fastcall {
() => {
// Module: crate::isa::x64::abi
// Provides: {"is_callee_save_fastcall"}
// Dependencies: {}
fn is_callee_save_fastcall (r : RealReg , enable_pinned_reg : bool) -> bool { use regs :: * ; match r . class () { RegClass :: Int => match r . hw_enc () { ENC_RBX | ENC_RBP | ENC_RSI | ENC_RDI | ENC_R12 | ENC_R13 | ENC_R14 => true , ENC_R15 => ! enable_pinned_reg , _ => false , } , RegClass :: Float => match r . hw_enc () { 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => true , _ => false , } , RegClass :: Vector => unreachable ! () , } }
};
}
