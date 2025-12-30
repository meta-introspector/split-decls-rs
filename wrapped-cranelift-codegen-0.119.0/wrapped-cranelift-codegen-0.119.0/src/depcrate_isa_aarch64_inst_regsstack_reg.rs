// Generated macro for stack_reg (function)
macro_rules! Depcrate_isa_aarch64_inst_regsstack_reg {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"stack_reg"}
// Dependencies: {}
# [doc = " Get a reference to the stack-pointer register."] pub fn stack_reg () -> Reg { let preg = PReg :: new (31 + 32 , RegClass :: Int) ; Reg :: from (VReg :: new (preg . index () , RegClass :: Int)) }
};
}
