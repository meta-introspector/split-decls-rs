// Generated macro for writable_vreg (function)
macro_rules! Depcrate_isa_aarch64_inst_regswritable_vreg {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"writable_vreg"}
// Dependencies: {}
# [doc = " Get a writable reference to a V-register."] # [cfg (test)] pub fn writable_vreg (num : u8) -> Writable < Reg > { Writable :: from_reg (vreg (num)) }
};
}
