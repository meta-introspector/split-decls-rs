// Generated macro for impl_157 (impl)
macro_rules! Depcrate_extensionsimpl_157 {
() => {
// Module: crate::extensions
// Provides: {"impl_157"}
// Dependencies: {}
impl < 'a > asn1 :: SimpleAsn1Readable < 'a > for KeyUsage < 'a > { const TAG : asn1 :: Tag = asn1 :: BitString :: TAG ; fn parse_data (data : & 'a [u8]) -> asn1 :: ParseResult < Self > { asn1 :: BitString :: parse_data (data) . map (Self) } }
};
}
