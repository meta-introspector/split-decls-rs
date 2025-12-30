// Generated macro for FromWritableReg (trait)
macro_rules! Depcrate_isa_x64_inst_argsFromWritableReg {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"FromWritableReg"}
// Dependencies: {}
# [doc = " An extension trait for converting `Writable<Reg>` to `Writable{Xmm,Gpr}`."] pub trait FromWritableReg : Sized { # [doc = " Convert `Writable<Reg>` to `Writable{Xmm,Gpr}`."] fn from_writable_reg (w : Writable < Reg >) -> Option < Self > ; }
};
}
