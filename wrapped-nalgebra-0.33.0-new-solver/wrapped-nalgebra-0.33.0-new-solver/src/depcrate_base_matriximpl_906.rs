// Generated macro for impl_906 (impl)
macro_rules! Depcrate_base_matriximpl_906 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_906"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < T > DMatrix < T > { # [doc = " Creates a new heap-allocated matrix from the given [`VecStorage`]."] # [doc = ""] # [doc = " This method exists primarily as a workaround for the fact that `from_data` can not"] # [doc = " work in `const fn` contexts."] pub const fn from_vec_storage (storage : VecStorage < T , Dyn , Dyn >) -> Self { unsafe { Self :: from_data_statically_unchecked (storage) } } }
};
}
