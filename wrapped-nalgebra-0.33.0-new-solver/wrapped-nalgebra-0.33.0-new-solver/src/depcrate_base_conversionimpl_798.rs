// Generated macro for impl_798 (impl)
macro_rules! Depcrate_base_conversionimpl_798 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_798"}
// Dependencies: {}
impl < T : Scalar + PrimitiveSimdValue , R : Dim , C : Dim > From < [OMatrix < T :: Element , R , C > ; 2] > for OMatrix < T , R , C > where T : From < [< T as SimdValue > :: Element ; 2] > , T :: Element : Scalar + SimdValue , DefaultAllocator : Allocator < R , C > , { # [inline] fn from (arr : [OMatrix < T :: Element , R , C > ; 2]) -> Self { let (nrows , ncols) = arr [0] . shape_generic () ; Self :: from_fn_generic (nrows , ncols , | i , j | { [arr [0] [(i , j)] . clone () , arr [1] [(i , j)] . clone ()] . into () }) } }
};
}
