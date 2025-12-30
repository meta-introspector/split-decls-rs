// Generated macro for Bidiagonal (struct)
macro_rules! Depcrate_linalg_bidiagonalBidiagonal {
() => {
// Module: crate::linalg::bidiagonal
// Provides: {"Bidiagonal"}
// Dependencies: {}
# [doc = " The bidiagonalization of a general matrix."] # [cfg_attr (feature = "serde-serialize-no-std" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (serialize = "DimMinimum<R, C>: DimSub<U1>,
         DefaultAllocator: Allocator<R, C>             +
                           Allocator<DimMinimum<R, C>> +
                           Allocator<DimDiff<DimMinimum<R, C>, U1>>,
         OMatrix<T, R, C>: Serialize,
         OVector<T, DimMinimum<R, C>>: Serialize,
         OVector<T, DimDiff<DimMinimum<R, C>, U1>>: Serialize")))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (deserialize = "DimMinimum<R, C>: DimSub<U1>,
         DefaultAllocator: Allocator<R, C>             +
                           Allocator<DimMinimum<R, C>> +
                           Allocator<DimDiff<DimMinimum<R, C>, U1>>,
         OMatrix<T, R, C>: Deserialize<'de>,
         OVector<T, DimMinimum<R, C>>: Deserialize<'de>,
         OVector<T, DimDiff<DimMinimum<R, C>, U1>>: Deserialize<'de>")))] # [derive (Clone , Debug)] pub struct Bidiagonal < T : ComplexField , R : DimMin < C > , C : Dim > where DimMinimum < R , C > : DimSub < U1 > , DefaultAllocator : Allocator < R , C > + Allocator < DimMinimum < R , C > > + Allocator < DimDiff < DimMinimum < R , C > , U1 > > , { uv : OMatrix < T , R , C > , # [doc = " The diagonal elements of the decomposed matrix."] diagonal : OVector < T , DimMinimum < R , C > > , # [doc = " The off-diagonal elements of the decomposed matrix."] off_diagonal : OVector < T , DimDiff < DimMinimum < R , C > , U1 > > , upper_diagonal : bool , }
};
}
