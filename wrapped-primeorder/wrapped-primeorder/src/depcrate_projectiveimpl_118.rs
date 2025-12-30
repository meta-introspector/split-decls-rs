// Generated macro for impl_118 (impl)
macro_rules! Depcrate_projectiveimpl_118 {
() => {
// Module: crate::projective
// Provides: {"impl_118"}
// Dependencies: {}
impl < C > SubAssign < & ProjectivePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn sub_assign (& mut self , rhs : & ProjectivePoint < C >) { * self = ProjectivePoint :: sub (self , rhs) ; } }
};
}
