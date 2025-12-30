// Generated macro for x_reg_range (function)
macro_rules! Depcrate_isa_riscv64_inst_regsx_reg_range {
() => {
// Module: crate::isa::riscv64::inst::regs
// Provides: {"x_reg_range"}
// Dependencies: {}
# [allow (dead_code)] pub (crate) fn x_reg_range (start : usize , end : usize) -> Vec < Writable < Reg > > { let mut regs = vec ! [] ; for i in start ..= end { regs . push (Writable :: from_reg (x_reg (i))) ; } regs }
};
}
