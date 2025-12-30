// Generated macro for impl_163 (impl)
macro_rules! Depcrate_verifyingimpl_163 {
() => {
// Module: crate::verifying
// Provides: {"impl_163"}
// Dependencies: {}
impl < C > PartialOrd for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
};
}
