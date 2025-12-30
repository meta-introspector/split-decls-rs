// Generated macro for xreg (function)
macro_rules! Depcrate_isa_aarch64_inst_regsxreg {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"xreg"}
// Dependencies: {}
# [doc = " Get a reference to an X-register (integer register). Do not use"] # [doc = " this for xsp / xzr; we have two special registers for those."] pub fn xreg (num : u8) -> Reg { Reg :: from (xreg_preg (num)) }
};
}
