// Generated macro for gpr_preg (function)
macro_rules! Depcrate_isa_s390x_inst_regsgpr_preg {
() => {
// Module: crate::isa::s390x::inst::regs
// Provides: {"gpr_preg"}
// Dependencies: {}
pub (crate) const fn gpr_preg (num : u8) -> PReg { assert ! (num < 16) ; PReg :: new (num as usize , RegClass :: Int) }
};
}
