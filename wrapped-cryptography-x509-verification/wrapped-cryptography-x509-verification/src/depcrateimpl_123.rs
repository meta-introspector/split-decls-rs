// Generated macro for impl_123 (impl)
macro_rules! Depcrateimpl_123 {
() => {
// Module: crate
// Provides: {"impl_123"}
// Dependencies: {}
impl < B : CryptoOps > From < asn1 :: ParseError > for ValidationError < '_ , B > { fn from (value : asn1 :: ParseError) -> Self { Self :: new (ValidationErrorKind :: Malformed (value)) } }
};
}
