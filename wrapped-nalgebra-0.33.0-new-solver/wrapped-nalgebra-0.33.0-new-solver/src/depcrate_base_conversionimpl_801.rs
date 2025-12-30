// Generated macro for impl_801 (impl)
macro_rules! Depcrate_base_conversionimpl_801 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_801"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue , R : Dim , C : Dim > From < [OMatrix < T :: Element , R , C > ; 16] > for OMatrix < T , R , C > where T : From < [< T as SimdValue > :: Element ; 16] > , T :: Element : Scalar + SimdValue , DefaultAllocator : Allocator < R , C > , { fn from (arr : [OMatrix < T :: Element , R , C > ; 16]) -> Self { let (nrows , ncols) = arr [0] . shape_generic () ; Self :: from_fn_generic (nrows , ncols , | i , j | { [arr [0] [(i , j)] . clone () , arr [1] [(i , j)] . clone () , arr [2] [(i , j)] . clone () , arr [3] [(i , j)] . clone () , arr [4] [(i , j)] . clone () , arr [5] [(i , j)] . clone () , arr [6] [(i , j)] . clone () , arr [7] [(i , j)] . clone () , arr [8] [(i , j)] . clone () , arr [9] [(i , j)] . clone () , arr [10] [(i , j)] . clone () , arr [11] [(i , j)] . clone () , arr [12] [(i , j)] . clone () , arr [13] [(i , j)] . clone () , arr [14] [(i , j)] . clone () , arr [15] [(i , j)] . clone () ,] . into () }) } }
};
}
