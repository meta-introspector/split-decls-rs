// Generated macro for impl_32 (impl)
macro_rules! Depcrate_affineimpl_32 {
() => {
// Module: crate::affine
// Provides: {"impl_32"}
// Dependencies: {}
impl < C > ConstantTimeEq for AffinePoint < C > where C : PrimeCurveParams , { fn ct_eq (& self , other : & Self) -> Choice { self . x . ct_eq (& other . x) & self . y . ct_eq (& other . y) & self . infinity . ct_eq (& other . infinity) } }
};
}
