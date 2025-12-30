// Generated macro for impl_771 (impl)
macro_rules! Depcrate_base_conversionimpl_771 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_771"}
// Dependencies: {}
impl < T : Scalar , const D : usize > From < [T ; D] > for RowSVector < T , D > where Const < D > : IsNotStaticOne , { # [inline] fn from (arr : [T ; D]) -> Self { SVector :: < T , D > :: from (arr) . transpose () } }
};
}
