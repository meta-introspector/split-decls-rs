// Generated macro for impl_905 (impl)
macro_rules! Depcrate_base_matriximpl_905 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_905"}
// Dependencies: {}
impl < T , const R : usize , const C : usize > SMatrix < T , R , C > { # [doc = " Creates a new statically-allocated matrix from the given [`ArrayStorage`]."] # [doc = ""] # [doc = " This method exists primarily as a workaround for the fact that `from_data` can not"] # [doc = " work in `const fn` contexts."] # [inline (always)] pub const fn from_array_storage (storage : ArrayStorage < T , R , C >) -> Self { unsafe { Self :: from_data_statically_unchecked (storage) } } }
};
}
