// Generated macro for is_callee_save_systemv (function)
macro_rules! Depcrate_isa_x64_abiis_callee_save_systemv {
() => {
// Module: crate::isa::x64::abi
// Provides: {"is_callee_save_systemv"}
// Dependencies: {}
fn is_callee_save_systemv (r : RealReg , enable_pinned_reg : bool) -> bool { use regs :: * ; match r . class () { RegClass :: Int => match r . hw_enc () { ENC_RBX | ENC_RBP | ENC_R12 | ENC_R13 | ENC_R14 => true , ENC_R15 => ! enable_pinned_reg , _ => false , } , RegClass :: Float => false , RegClass :: Vector => unreachable ! () , } }
};
}
