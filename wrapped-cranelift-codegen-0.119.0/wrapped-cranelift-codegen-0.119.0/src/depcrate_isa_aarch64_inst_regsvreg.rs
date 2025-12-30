// Generated macro for vreg (function)
macro_rules! Depcrate_isa_aarch64_inst_regsvreg {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"vreg"}
// Dependencies: {}
# [doc = " Get a reference to a V-register (vector/FP register)."] pub fn vreg (num : u8) -> Reg { Reg :: from (vreg_preg (num)) }
};
}
