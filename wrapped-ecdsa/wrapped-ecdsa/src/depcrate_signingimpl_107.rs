// Generated macro for impl_107 (impl)
macro_rules! Depcrate_signingimpl_107 {
() => {
// Module: crate::signing
// Provides: {"impl_107"}
// Dependencies: {}
impl < C > Debug for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SigningKey") . finish_non_exhaustive () } }
};
}
