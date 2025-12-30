// Generated macro for writable_stack_reg (function)
macro_rules! Depcrate_isa_s390x_inst_regswritable_stack_reg {
() => {
// Module: crate::isa::s390x::inst::regs
// Provides: {"writable_stack_reg"}
// Dependencies: {}
# [doc = " Get a writable reference to the stack-pointer register."] pub fn writable_stack_reg () -> Writable < Reg > { Writable :: from_reg (stack_reg ()) }
};
}
