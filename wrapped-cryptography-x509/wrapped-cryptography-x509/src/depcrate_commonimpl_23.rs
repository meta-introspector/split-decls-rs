// Generated macro for impl_23 (impl)
macro_rules! Depcrate_commonimpl_23 {
() => {
// Module: crate::common
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'a > asn1 :: Asn1Readable < 'a > for RawTlv < 'a > { fn parse (parser : & mut asn1 :: Parser < 'a >) -> asn1 :: ParseResult < Self > { let tlv = parser . read_element :: < asn1 :: Tlv < 'a > > () ? ; Ok (RawTlv :: new (tlv . tag () , tlv . data ())) } fn can_parse (_tag : asn1 :: Tag) -> bool { true } }
};
}
