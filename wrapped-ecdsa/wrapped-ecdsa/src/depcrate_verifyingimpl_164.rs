// Generated macro for impl_164 (impl)
macro_rules! Depcrate_verifyingimpl_164 {
() => {
// Module: crate::verifying
// Provides: {"impl_164"}
// Dependencies: {}
impl < C > Ord for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { fn cmp (& self , other : & Self) -> Ordering { self . inner . cmp (& other . inner) } }
};
}
