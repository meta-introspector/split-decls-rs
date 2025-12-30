// Generated macro for vreg_preg (function)
macro_rules! Depcrate_isa_aarch64_inst_regsvreg_preg {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"vreg_preg"}
// Dependencies: {}
# [doc = " Get the given V-register as a PReg."] pub (crate) const fn vreg_preg (num : u8) -> PReg { assert ! (num < 32) ; PReg :: new (num as usize , RegClass :: Float) }
};
}
