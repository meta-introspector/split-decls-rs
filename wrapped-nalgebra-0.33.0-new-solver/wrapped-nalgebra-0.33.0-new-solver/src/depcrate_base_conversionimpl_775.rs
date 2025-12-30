// Generated macro for impl_775 (impl)
macro_rules! Depcrate_base_conversionimpl_775 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_775"}
// Dependencies: {}
impl < T : Scalar , const R : usize , const C : usize > From < [[T ; R] ; C] > for SMatrix < T , R , C > { # [inline] fn from (arr : [[T ; R] ; C]) -> Self { unsafe { Self :: from_data_statically_unchecked (ArrayStorage (arr)) } } }
};
}
