// Generated macro for writable_fp_reg (function)
macro_rules! Depcrate_isa_aarch64_inst_regswritable_fp_reg {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"writable_fp_reg"}
// Dependencies: {}
# [doc = " Get a writable reference to the frame pointer."] pub fn writable_fp_reg () -> Writable < Reg > { Writable :: from_reg (fp_reg ()) }
};
}
