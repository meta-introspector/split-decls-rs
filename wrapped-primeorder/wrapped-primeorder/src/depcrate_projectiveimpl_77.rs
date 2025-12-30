// Generated macro for impl_77 (impl)
macro_rules! Depcrate_projectiveimpl_77 {
() => {
// Module: crate::projective
// Provides: {"impl_77"}
// Dependencies: {}
impl < C > From < AffinePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn from (p : AffinePoint < C >) -> Self { let projective = ProjectivePoint { x : p . x , y : p . y , z : C :: FieldElement :: ONE , } ; Self :: conditional_select (& projective , & Self :: IDENTITY , p . is_identity ()) } }
};
}
