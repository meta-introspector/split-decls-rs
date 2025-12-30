// Generated macro for axcpy_uninit (function)
macro_rules! Depcrate_base_blas_uninitaxcpy_uninit {
() => {
// Module: crate::base::blas_uninit
// Provides: {"axcpy_uninit"}
// Dependencies: {}
# [doc = " Computes `y = a * x * c + b * y`."] # [doc = ""] # [doc = " If `b` is zero, `y` is never read from and may be uninitialized."] # [doc = ""] # [doc = " # Safety"] # [doc = " This is UB if b != 0 and any component of `y` is uninitialized."] # [inline (always)] # [allow (clippy :: many_single_char_names)] pub unsafe fn axcpy_uninit < Status , T , D1 : Dim , D2 : Dim , SA , SB > (status : Status , y : & mut Vector < Status :: Value , D1 , SA > , a : T , x : & Vector < T , D2 , SB > , c : T , b : T ,) where T : Scalar + Zero + ClosedAddAssign + ClosedMulAssign , SA : RawStorageMut < Status :: Value , D1 > , SB : RawStorage < T , D2 > , ShapeConstraint : DimEq < D1 , D2 > , Status : InitStatus < T > , { assert_eq ! (y . nrows () , x . nrows () , "Axcpy: mismatched vector shapes.") ; let rstride1 = y . strides () . 0 ; let rstride2 = x . strides () . 0 ; let y = y . data . as_mut_slice_unchecked () ; let x = x . data . as_slice_unchecked () ; if ! b . is_zero () { array_axcpy (status , y , a , x , c , b , rstride1 , rstride2 , x . len ()) ; } else { array_axc (status , y , a , x , c , rstride1 , rstride2 , x . len ()) ; } }
};
}
