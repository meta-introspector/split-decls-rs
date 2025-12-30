// Generated macro for impl_44 (impl)
macro_rules! Depcrate_arrayvecimpl_44 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_44"}
// Dependencies: {}
impl < T , const N : usize > AsMut < [Option < T >] > for ArrayVec < T , N > { fn as_mut (& mut self) -> & mut [Option < T >] { & mut self . elements [.. self . length] } }
};
}
