// Generated macro for impl_71 (impl)
macro_rules! Depcrate_projectiveimpl_71 {
() => {
// Module: crate::projective
// Provides: {"impl_71"}
// Dependencies: {}
impl < C > ConditionallySelectable for ProjectivePoint < C > where C : PrimeCurveParams , { # [inline (always)] fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { Self { x : C :: FieldElement :: conditional_select (& a . x , & b . x , choice) , y : C :: FieldElement :: conditional_select (& a . y , & b . y , choice) , z : C :: FieldElement :: conditional_select (& a . z , & b . z , choice) , } } }
};
}
