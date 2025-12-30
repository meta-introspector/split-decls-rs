// Generated macro for impl_106 (impl)
macro_rules! Depcrate_projectiveimpl_106 {
() => {
// Module: crate::projective
// Provides: {"impl_106"}
// Dependencies: {}
impl < C > AddAssign < & ProjectivePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn add_assign (& mut self , rhs : & ProjectivePoint < C >) { * self = ProjectivePoint :: add (self , rhs) ; } }
};
}
