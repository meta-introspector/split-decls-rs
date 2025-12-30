// Generated macro for gpr (function)
macro_rules! Depcrate_isa_s390x_inst_regsgpr {
() => {
// Module: crate::isa::s390x::inst::regs
// Provides: {"gpr"}
// Dependencies: {}
# [doc = " Get a reference to a GPR (integer register)."] pub fn gpr (num : u8) -> Reg { Reg :: from (gpr_preg (num)) }
};
}
