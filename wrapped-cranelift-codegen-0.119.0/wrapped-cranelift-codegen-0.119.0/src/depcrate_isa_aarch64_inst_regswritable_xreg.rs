// Generated macro for writable_xreg (function)
macro_rules! Depcrate_isa_aarch64_inst_regswritable_xreg {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"writable_xreg"}
// Dependencies: {}
# [doc = " Get a writable reference to an X-register."] pub fn writable_xreg (num : u8) -> Writable < Reg > { Writable :: from_reg (xreg (num)) }
};
}
