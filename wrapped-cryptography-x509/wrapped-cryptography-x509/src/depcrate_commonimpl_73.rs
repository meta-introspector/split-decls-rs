// Generated macro for impl_73 (impl)
macro_rules! Depcrate_commonimpl_73 {
() => {
// Module: crate::common
// Provides: {"impl_73"}
// Dependencies: {}
impl < 'a , T : asn1 :: Asn1Readable < 'a > > asn1 :: Asn1Readable < 'a > for WithTlv < 'a , T > { fn parse (p : & mut asn1 :: Parser < 'a >) -> asn1 :: ParseResult < Self > { let tlv = p . read_element :: < asn1 :: Tlv < 'a > > () ? ; Ok (Self { tlv , value : tlv . parse () ? , }) } fn can_parse (t : asn1 :: Tag) -> bool { T :: can_parse (t) } }
};
}
