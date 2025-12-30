// Generated macro for impl_772 (impl)
macro_rules! Depcrate_base_conversionimpl_772 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_772"}
// Dependencies: {}
impl < T : Scalar , const D : usize > From < RowSVector < T , D > > for [T ; D] where Const < D > : IsNotStaticOne , { # [inline] fn from (vec : RowSVector < T , D >) -> [T ; D] { vec . transpose () . into () } }
};
}
