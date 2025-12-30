// Generated macro for writable_link_reg (function)
macro_rules! Depcrate_isa_riscv64_inst_regswritable_link_reg {
() => {
// Module: crate::isa::riscv64::inst::regs
// Provides: {"writable_link_reg"}
// Dependencies: {}
# [doc = " Get a writable reference to the link register."] # [inline] pub fn writable_link_reg () -> Writable < Reg > { Writable :: from_reg (link_reg ()) }
};
}
