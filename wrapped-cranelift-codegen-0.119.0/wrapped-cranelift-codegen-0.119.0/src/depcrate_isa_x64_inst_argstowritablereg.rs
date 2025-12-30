// Generated macro for ToWritableReg (trait)
macro_rules! Depcrate_isa_x64_inst_argsToWritableReg {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"ToWritableReg"}
// Dependencies: {}
# [doc = " An extension trait for converting `Writable{Xmm,Gpr}` to `Writable<Reg>`."] pub trait ToWritableReg { # [doc = " Convert `Writable{Xmm,Gpr}` to `Writable<Reg>`."] fn to_writable_reg (& self) -> Writable < Reg > ; }
};
}
