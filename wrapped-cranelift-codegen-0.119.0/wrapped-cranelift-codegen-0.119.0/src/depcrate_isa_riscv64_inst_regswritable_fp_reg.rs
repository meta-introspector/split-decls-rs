// Generated macro for writable_fp_reg (function)
macro_rules! Depcrate_isa_riscv64_inst_regswritable_fp_reg {
() => {
// Module: crate::isa::riscv64::inst::regs
// Provides: {"writable_fp_reg"}
// Dependencies: {}
# [doc = " Get a writable reference to the frame pointer."] # [inline] pub fn writable_fp_reg () -> Writable < Reg > { Writable :: from_reg (fp_reg ()) }
};
}
