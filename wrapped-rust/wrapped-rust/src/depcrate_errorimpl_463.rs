// Generated macro for impl_463 (impl)
macro_rules! Depcrate_errorimpl_463 {
() => {
// Module: crate::error
// Provides: {"impl_463"}
// Dependencies: {}
impl From < asn1 :: WriteError > for CryptographyError { fn from (e : asn1 :: WriteError) -> CryptographyError { CryptographyError :: Asn1Write (e) } }
};
}
