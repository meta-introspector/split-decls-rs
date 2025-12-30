// Generated macro for impl_106 (impl)
macro_rules! Depcrate_signingimpl_106 {
() => {
// Module: crate::signing
// Provides: {"impl_106"}
// Dependencies: {}
impl < C > ConstantTimeEq for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn ct_eq (& self , other : & Self) -> Choice { self . secret_scalar . ct_eq (& other . secret_scalar) } }
};
}
