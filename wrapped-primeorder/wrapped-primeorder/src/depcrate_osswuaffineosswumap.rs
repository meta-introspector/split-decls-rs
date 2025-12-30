// Generated macro for AffineOsswuMap (trait)
macro_rules! Depcrate_osswuAffineOsswuMap {
() => {
// Module: crate::osswu
// Provides: {"AffineOsswuMap"}
// Dependencies: {}
# [doc = " [`OsswuMap`] for [`AffinePoint`]."] pub trait AffineOsswuMap < C : PrimeCurveParams < FieldElement : OsswuMap > > { # [doc = " [`OsswuMap::osswu()`] to [`AffinePoint`]."] fn osswu (u : & C :: FieldElement) -> Self ; }
};
}
