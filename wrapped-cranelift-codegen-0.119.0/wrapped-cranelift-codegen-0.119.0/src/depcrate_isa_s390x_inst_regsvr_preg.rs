// Generated macro for vr_preg (function)
macro_rules! Depcrate_isa_s390x_inst_regsvr_preg {
() => {
// Module: crate::isa::s390x::inst::regs
// Provides: {"vr_preg"}
// Dependencies: {}
pub (crate) const fn vr_preg (num : u8) -> PReg { assert ! (num < 32) ; PReg :: new (num as usize , RegClass :: Float) }
};
}
