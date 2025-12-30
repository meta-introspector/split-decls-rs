// Generated macro for writable_link_reg (function)
macro_rules! Depcrate_isa_aarch64_inst_regswritable_link_reg {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"writable_link_reg"}
// Dependencies: {}
# [doc = " Get a writable reference to the link register."] pub fn writable_link_reg () -> Writable < Reg > { Writable :: from_reg (link_reg ()) }
};
}
