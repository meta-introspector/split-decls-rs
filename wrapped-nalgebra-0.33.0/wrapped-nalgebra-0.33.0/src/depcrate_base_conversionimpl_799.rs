// Generated macro for impl_799 (impl)
macro_rules! Depcrate_base_conversionimpl_799 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_799"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue , R : Dim , C : Dim > From < [OMatrix < T :: Element , R , C > ; 4] > for OMatrix < T , R , C > where T : From < [< T as SimdValue > :: Element ; 4] > , T :: Element : Scalar + SimdValue , DefaultAllocator : Allocator < R , C > , { # [inline] fn from (arr : [OMatrix < T :: Element , R , C > ; 4]) -> Self { let (nrows , ncols) = arr [0] . shape_generic () ; Self :: from_fn_generic (nrows , ncols , | i , j | { [arr [0] [(i , j)] . clone () , arr [1] [(i , j)] . clone () , arr [2] [(i , j)] . clone () , arr [3] [(i , j)] . clone () ,] . into () }) } }
};
}
