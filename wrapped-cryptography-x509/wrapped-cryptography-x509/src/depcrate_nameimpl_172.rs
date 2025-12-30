// Generated macro for impl_172 (impl)
macro_rules! Depcrate_nameimpl_172 {
() => {
// Module: crate::name
// Provides: {"impl_172"}
// Dependencies: {}
impl < 'a > asn1 :: SimpleAsn1Readable < 'a > for UnvalidatedIA5String < 'a > { const TAG : asn1 :: Tag = asn1 :: IA5String :: TAG ; fn parse_data (data : & 'a [u8]) -> asn1 :: ParseResult < Self > { Ok (UnvalidatedIA5String (std :: str :: from_utf8 (data) . map_err (| _ | asn1 :: ParseError :: new (asn1 :: ParseErrorKind :: InvalidValue) ,) ?)) } }
};
}
