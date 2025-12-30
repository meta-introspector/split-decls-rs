// Generated macro for writable_zero_reg (function)
macro_rules! Depcrate_isa_aarch64_inst_regswritable_zero_reg {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"writable_zero_reg"}
// Dependencies: {}
# [doc = " Get a writable reference to the zero-register (this discards a result)."] pub fn writable_zero_reg () -> Writable < Reg > { Writable :: from_reg (zero_reg ()) }
};
}
