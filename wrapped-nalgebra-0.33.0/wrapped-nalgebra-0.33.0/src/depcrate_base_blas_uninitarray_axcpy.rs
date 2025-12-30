// Generated macro for array_axcpy (function)
macro_rules! Depcrate_base_blas_uninitarray_axcpy {
() => {
// Module: crate::base::blas_uninit
// Provides: {"array_axcpy"}
// Dependencies: {}
# [allow (clippy :: too_many_arguments)] unsafe fn array_axcpy < Status , T > (_ : Status , y : & mut [Status :: Value] , a : T , x : & [T] , c : T , beta : T , stride1 : usize , stride2 : usize , len : usize ,) where Status : InitStatus < T > , T : Scalar + Zero + ClosedAddAssign + ClosedMulAssign , { for i in 0 .. len { let y = Status :: assume_init_mut (y . get_unchecked_mut (i * stride1)) ; * y = a . clone () * x . get_unchecked (i * stride2) . clone () * c . clone () + beta . clone () * y . clone () ; } }
};
}
