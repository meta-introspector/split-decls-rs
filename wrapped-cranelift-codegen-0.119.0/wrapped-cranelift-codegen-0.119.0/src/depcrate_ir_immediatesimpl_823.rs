// Generated macro for impl_823 (impl)
macro_rules! Depcrate_ir_immediatesimpl_823 {
() => {
// Module: crate::ir::immediates
// Provides: {"impl_823"}
// Dependencies: {}
impl V128Imm { # [doc = " Iterate over the bytes in the constant."] pub fn bytes (& self) -> impl Iterator < Item = & u8 > { self . 0 . iter () } # [doc = " Convert the immediate into a vector."] pub fn to_vec (self) -> Vec < u8 > { self . 0 . to_vec () } # [doc = " Convert the immediate into a slice."] pub fn as_slice (& self) -> & [u8] { & self . 0 [..] } }
};
}
