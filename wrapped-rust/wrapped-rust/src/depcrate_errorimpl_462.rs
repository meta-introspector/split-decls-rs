// Generated macro for impl_462 (impl)
macro_rules! Depcrate_errorimpl_462 {
() => {
// Module: crate::error
// Provides: {"impl_462"}
// Dependencies: {}
impl From < asn1 :: ParseError > for CryptographyError { fn from (e : asn1 :: ParseError) -> CryptographyError { CryptographyError :: Asn1Parse (e) } }
};
}
