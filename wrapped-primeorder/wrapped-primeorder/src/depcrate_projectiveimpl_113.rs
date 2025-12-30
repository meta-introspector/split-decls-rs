// Generated macro for impl_113 (impl)
macro_rules! Depcrate_projectiveimpl_113 {
() => {
// Module: crate::projective
// Provides: {"impl_113"}
// Dependencies: {}
impl < 'a , C > Sum < & 'a ProjectivePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn sum < I : Iterator < Item = & 'a ProjectivePoint < C > > > (iter : I) -> Self { iter . cloned () . sum () } }
};
}
