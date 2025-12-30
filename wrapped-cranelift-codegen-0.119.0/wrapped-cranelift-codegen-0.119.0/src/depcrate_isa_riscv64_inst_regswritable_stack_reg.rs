// Generated macro for writable_stack_reg (function)
macro_rules! Depcrate_isa_riscv64_inst_regswritable_stack_reg {
() => {
// Module: crate::isa::riscv64::inst::regs
// Provides: {"writable_stack_reg"}
// Dependencies: {}
# [doc = " Get a writable reference to the stack-pointer register."] # [inline] pub fn writable_stack_reg () -> Writable < Reg > { Writable :: from_reg (stack_reg ()) }
};
}
