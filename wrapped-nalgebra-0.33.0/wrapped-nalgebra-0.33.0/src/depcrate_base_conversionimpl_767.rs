// Generated macro for impl_767 (impl)
macro_rules! Depcrate_base_conversionimpl_767 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_767"}
// Dependencies: {}
impl < T : Scalar , const D : usize > From < [T ; D] > for SVector < T , D > { # [inline] fn from (arr : [T ; D]) -> Self { unsafe { Self :: from_data_statically_unchecked (ArrayStorage ([arr ; 1])) } } }
};
}
