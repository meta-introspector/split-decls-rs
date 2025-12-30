// Generated macro for impl_7 (impl)
macro_rules! Depcrate_osswuimpl_7 {
() => {
// Module: crate::osswu
// Provides: {"impl_7"}
// Dependencies: {}
impl < C : PrimeCurveParams < FieldElement : OsswuMap > > AffineOsswuMap < C > for AffinePoint < C > { fn osswu (u : & < C as PrimeCurveParams > :: FieldElement) -> Self { let (x , y) = u . osswu () ; Self { x , y , infinity : 0 } } }
};
}
