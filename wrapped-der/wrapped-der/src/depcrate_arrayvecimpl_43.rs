// Generated macro for impl_43 (impl)
macro_rules! Depcrate_arrayvecimpl_43 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_43"}
// Dependencies: {}
impl < T , const N : usize > AsRef < [Option < T >] > for ArrayVec < T , N > { fn as_ref (& self) -> & [Option < T >] { & self . elements [.. self . length] } }
};
}
