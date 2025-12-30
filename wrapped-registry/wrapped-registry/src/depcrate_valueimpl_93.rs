// Generated macro for impl_93 (impl)
macro_rules! Depcrate_valueimpl_93 {
() => {
// Module: crate::value
// Provides: {"impl_93"}
// Dependencies: {}
impl < const N : usize > From < [u8 ; N] > for Value { fn from (from : [u8 ; N]) -> Self { Self { data : Data :: from_slice (& from) , ty : Type :: Bytes , } } }
};
}
