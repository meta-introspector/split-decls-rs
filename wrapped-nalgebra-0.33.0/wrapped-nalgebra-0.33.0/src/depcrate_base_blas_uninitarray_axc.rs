// Generated macro for array_axc (function)
macro_rules! Depcrate_base_blas_uninitarray_axc {
() => {
// Module: crate::base::blas_uninit
// Provides: {"array_axc"}
// Dependencies: {}
fn array_axc < Status , T > (_ : Status , y : & mut [Status :: Value] , a : T , x : & [T] , c : T , stride1 : usize , stride2 : usize , len : usize ,) where Status : InitStatus < T > , T : Scalar + Zero + ClosedAddAssign + ClosedMulAssign , { for i in 0 .. len { unsafe { Status :: init (y . get_unchecked_mut (i * stride1) , a . clone () * x . get_unchecked (i * stride2) . clone () * c . clone () ,) ; } } }
};
}
