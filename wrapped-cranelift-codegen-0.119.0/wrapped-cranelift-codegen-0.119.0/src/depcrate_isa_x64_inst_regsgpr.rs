// Generated macro for gpr (function)
macro_rules! Depcrate_isa_x64_inst_regsgpr {
() => {
// Module: crate::isa::x64::inst::regs
// Provides: {"gpr"}
// Dependencies: {}
fn gpr (enc : u8) -> Reg { let preg = gpr_preg (enc) ; Reg :: from (VReg :: new (preg . index () , RegClass :: Int)) }
};
}
