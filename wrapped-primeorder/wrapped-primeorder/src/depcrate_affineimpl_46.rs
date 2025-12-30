// Generated macro for impl_46 (impl)
macro_rules! Depcrate_affineimpl_46 {
() => {
// Module: crate::affine
// Provides: {"impl_46"}
// Dependencies: {}
impl < C > PartialEq for AffinePoint < C > where C : PrimeCurveParams , { fn eq (& self , other : & Self) -> bool { self . ct_eq (other) . into () } }
};
}
