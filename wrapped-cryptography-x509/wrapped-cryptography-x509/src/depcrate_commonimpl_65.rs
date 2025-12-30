// Generated macro for impl_65 (impl)
macro_rules! Depcrate_commonimpl_65 {
() => {
// Module: crate::common
// Provides: {"impl_65"}
// Dependencies: {}
impl < 'a > asn1 :: SimpleAsn1Readable < 'a > for UnvalidatedVisibleString < 'a > { const TAG : asn1 :: Tag = < asn1 :: VisibleString < '_ > as asn1 :: SimpleAsn1Readable > :: TAG ; fn parse_data (data : & 'a [u8]) -> asn1 :: ParseResult < Self > { Ok (UnvalidatedVisibleString (std :: str :: from_utf8 (data) . map_err (| _ | asn1 :: ParseError :: new (asn1 :: ParseErrorKind :: InvalidValue)) ? ,)) } }
};
}
