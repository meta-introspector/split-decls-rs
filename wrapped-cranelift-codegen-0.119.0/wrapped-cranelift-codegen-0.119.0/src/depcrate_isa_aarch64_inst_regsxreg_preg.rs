// Generated macro for xreg_preg (function)
macro_rules! Depcrate_isa_aarch64_inst_regsxreg_preg {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"xreg_preg"}
// Dependencies: {}
# [doc = " Get the given X-register as a PReg."] pub (crate) const fn xreg_preg (num : u8) -> PReg { assert ! (num < 31) ; PReg :: new (num as usize , RegClass :: Int) }
};
}
