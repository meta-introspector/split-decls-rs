// Generated macro for impl_134 (impl)
macro_rules! Depcrate_extensionsimpl_134 {
() => {
// Module: crate::extensions
// Provides: {"impl_134"}
// Dependencies: {}
impl < 'a > Extension < 'a > { pub fn value < T : asn1 :: Asn1Readable < 'a > > (& self) -> asn1 :: ParseResult < T > { asn1 :: parse_single (self . extn_value) } }
};
}
