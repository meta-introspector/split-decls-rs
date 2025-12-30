// Generated macro for impl_117 (impl)
macro_rules! Depcrate_projectiveimpl_117 {
() => {
// Module: crate::projective
// Provides: {"impl_117"}
// Dependencies: {}
impl < C > SubAssign < ProjectivePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn sub_assign (& mut self , rhs : ProjectivePoint < C >) { * self = ProjectivePoint :: sub (self , & rhs) ; } }
};
}
