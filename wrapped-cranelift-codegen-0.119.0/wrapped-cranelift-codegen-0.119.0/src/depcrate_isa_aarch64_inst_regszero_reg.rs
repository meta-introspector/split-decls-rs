// Generated macro for zero_reg (function)
macro_rules! Depcrate_isa_aarch64_inst_regszero_reg {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"zero_reg"}
// Dependencies: {}
# [doc = " Get a reference to the zero-register."] pub fn zero_reg () -> Reg { let preg = PReg :: new (31 , RegClass :: Int) ; Reg :: from (VReg :: new (preg . index () , RegClass :: Int)) }
};
}
