// Generated macro for UDU (struct)
macro_rules! Depcrate_linalg_uduUDU {
() => {
// Module: crate::linalg::udu
// Provides: {"UDU"}
// Dependencies: {}
# [doc = " UDU factorization."] # [cfg_attr (feature = "serde-serialize-no-std" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (serialize = "OVector<T, D>: Serialize, OMatrix<T, D, D>: Serialize")))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (deserialize = "OVector<T, D>: Deserialize<'de>, OMatrix<T, D, D>: Deserialize<'de>")))] # [derive (Clone , Debug)] pub struct UDU < T : RealField , D : Dim > where DefaultAllocator : Allocator < D > + Allocator < D , D > , { # [doc = " The upper triangular matrix resulting from the factorization"] pub u : OMatrix < T , D , D > , # [doc = " The diagonal matrix resulting from the factorization"] pub d : OVector < T , D > , }
};
}
