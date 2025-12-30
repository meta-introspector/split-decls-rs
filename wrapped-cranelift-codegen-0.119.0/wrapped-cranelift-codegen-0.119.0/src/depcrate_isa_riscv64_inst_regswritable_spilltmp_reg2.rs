// Generated macro for writable_spilltmp_reg2 (function)
macro_rules! Depcrate_isa_riscv64_inst_regswritable_spilltmp_reg2 {
() => {
// Module: crate::isa::riscv64::inst::regs
// Provides: {"writable_spilltmp_reg2"}
// Dependencies: {}
# [doc = " Get a writable reference to the spilltmp2 reg."] # [inline] pub fn writable_spilltmp_reg2 () -> Writable < Reg > { Writable :: from_reg (spilltmp_reg2 ()) }
};
}
