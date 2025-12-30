// Generated macro for impl_105 (impl)
macro_rules! Depcrate_projectiveimpl_105 {
() => {
// Module: crate::projective
// Provides: {"impl_105"}
// Dependencies: {}
impl < C > AddAssign < ProjectivePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn add_assign (& mut self , rhs : ProjectivePoint < C >) { * self = ProjectivePoint :: add (self , & rhs) ; } }
};
}
