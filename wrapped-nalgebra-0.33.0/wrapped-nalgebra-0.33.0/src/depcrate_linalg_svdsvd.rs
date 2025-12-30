// Generated macro for SVD (struct)
macro_rules! Depcrate_linalg_svdSVD {
() => {
// Module: crate::linalg::svd
// Provides: {"SVD"}
// Dependencies: {}
# [doc = " Singular Value Decomposition of a general matrix."] # [cfg_attr (feature = "serde-serialize-no-std" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (serialize = "DefaultAllocator: Allocator<DimMinimum<R, C>>    +
                           Allocator<DimMinimum<R, C>, C> +
                           Allocator<R, DimMinimum<R, C>>,
         OMatrix<T, R, DimMinimum<R, C>>: Serialize,
         OMatrix<T, DimMinimum<R, C>, C>: Serialize,
         OVector<T::RealField, DimMinimum<R, C>>: Serialize")))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (deserialize = "DefaultAllocator: Allocator<DimMinimum<R, C>> +
                           Allocator<DimMinimum<R, C>, C> +
                           Allocator<R, DimMinimum<R, C>>,
         OMatrix<T, R, DimMinimum<R, C>>: Deserialize<'de>,
         OMatrix<T, DimMinimum<R, C>, C>: Deserialize<'de>,
         OVector<T::RealField, DimMinimum<R, C>>: Deserialize<'de>")))] # [derive (Clone , Debug)] pub struct SVD < T : ComplexField , R : DimMin < C > , C : Dim > where DefaultAllocator : Allocator < DimMinimum < R , C > , C > + Allocator < R , DimMinimum < R , C > > + Allocator < DimMinimum < R , C > > , { # [doc = " The left-singular vectors `U` of this SVD."] pub u : Option < OMatrix < T , R , DimMinimum < R , C > > > , # [doc = " The right-singular vectors `V^t` of this SVD."] pub v_t : Option < OMatrix < T , DimMinimum < R , C > , C > > , # [doc = " The singular values of this SVD."] pub singular_values : OVector < T :: RealField , DimMinimum < R , C > > , }
};
}
