// Generated macro for impl_99 (impl)
macro_rules! Depcrate_projectiveimpl_99 {
() => {
// Module: crate::projective
// Provides: {"impl_99"}
// Dependencies: {}
# [doc = " The constant-time alternative is available at [`NonIdentity::new()`]."] impl < C > TryFrom < ProjectivePoint < C > > for NonIdentity < ProjectivePoint < C > > where C : PrimeCurveParams , { type Error = Error ; fn try_from (point : ProjectivePoint < C >) -> Result < Self > { NonIdentity :: new (point) . into_option () . ok_or (Error) } }
};
}
