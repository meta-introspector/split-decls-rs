// Generated macro for gemv_uninit (function)
macro_rules! Depcrate_base_blas_uninitgemv_uninit {
() => {
// Module: crate::base::blas_uninit
// Provides: {"gemv_uninit"}
// Dependencies: {}
# [doc = " Computes `y = alpha * a * x + beta * y`, where `a` is a matrix, `x` a vector, and"] # [doc = " `alpha, beta` two scalars."] # [doc = ""] # [doc = " If `beta` is zero, `y` is never read from and may be uninitialized."] # [doc = ""] # [doc = " # Safety"] # [doc = " This is UB if beta != 0 and any component of `y` is uninitialized."] # [inline (always)] pub unsafe fn gemv_uninit < Status , T , D1 : Dim , R2 : Dim , C2 : Dim , D3 : Dim , SA , SB , SC > (status : Status , y : & mut Vector < Status :: Value , D1 , SA > , alpha : T , a : & Matrix < T , R2 , C2 , SB > , x : & Vector < T , D3 , SC > , beta : T ,) where Status : InitStatus < T > , T : Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign , SA : RawStorageMut < Status :: Value , D1 > , SB : RawStorage < T , R2 , C2 > , SC : RawStorage < T , D3 > , ShapeConstraint : DimEq < D1 , R2 > + AreMultipliable < R2 , C2 , D3 , U1 > , { let dim1 = y . nrows () ; let (nrows2 , ncols2) = a . shape () ; let dim3 = x . nrows () ; assert ! (ncols2 == dim3 && dim1 == nrows2 , "Gemv: dimensions mismatch.") ; if ncols2 == 0 { if beta . is_zero () { y . apply (| e | Status :: init (e , T :: zero ())) ; } else { y . apply (| e | * Status :: assume_init_mut (e) *= beta . clone ()) ; } return ; } let col2 = a . column (0) ; let val = x . vget_unchecked (0) . clone () ; axcpy_uninit (status , y , alpha . clone () , & col2 , val , beta) ; for j in 1 .. ncols2 { let col2 = a . column (j) ; let val = x . vget_unchecked (j) . clone () ; axcpy_uninit (status , y , alpha . clone () , & col2 , val , T :: one ()) ; } }
};
}
