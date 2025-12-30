// Generated macro for impl_110 (impl)
macro_rules! Depcrate_signingimpl_110 {
() => {
// Module: crate::signing
// Provides: {"impl_110"}
// Dependencies: {}
impl < C > PartialEq for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn eq (& self , other : & SigningKey < C >) -> bool { self . ct_eq (other) . into () } }
};
}
