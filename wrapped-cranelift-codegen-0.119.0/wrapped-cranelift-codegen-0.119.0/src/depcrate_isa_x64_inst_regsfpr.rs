// Generated macro for fpr (function)
macro_rules! Depcrate_isa_x64_inst_regsfpr {
() => {
// Module: crate::isa::x64::inst::regs
// Provides: {"fpr"}
// Dependencies: {}
fn fpr (enc : u8) -> Reg { let preg = fpr_preg (enc) ; Reg :: from (VReg :: new (preg . index () , RegClass :: Float)) }
};
}
