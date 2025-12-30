// Generated macro for QR (struct)
macro_rules! Depcrate_linalg_qrQR {
() => {
// Module: crate::linalg::qr
// Provides: {"QR"}
// Dependencies: {}
# [doc = " The QR decomposition of a general matrix."] # [cfg_attr (feature = "serde-serialize-no-std" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (serialize = "DefaultAllocator: Allocator<R, C> +
                           Allocator<DimMinimum<R, C>>,
         OMatrix<T, R, C>: Serialize,
         OVector<T, DimMinimum<R, C>>: Serialize")))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (deserialize = "DefaultAllocator: Allocator<R, C> +
                           Allocator<DimMinimum<R, C>>,
         OMatrix<T, R, C>: Deserialize<'de>,
         OVector<T, DimMinimum<R, C>>: Deserialize<'de>")))] # [derive (Clone , Debug)] pub struct QR < T : ComplexField , R : DimMin < C > , C : Dim > where DefaultAllocator : Allocator < R , C > + Allocator < DimMinimum < R , C > > , { qr : OMatrix < T , R , C > , diag : OVector < T , DimMinimum < R , C > > , }
};
}
