// Generated macro for writable_spilltmp_reg (function)
macro_rules! Depcrate_isa_aarch64_inst_regswritable_spilltmp_reg {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"writable_spilltmp_reg"}
// Dependencies: {}
# [doc = " Get a writable reference to the spilltmp reg."] pub fn writable_spilltmp_reg () -> Writable < Reg > { Writable :: from_reg (spilltmp_reg ()) }
};
}
