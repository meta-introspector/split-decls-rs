// Generated macro for check_attribute_length (function)
macro_rules! Depcrate_csrcheck_attribute_length {
() => {
// Module: crate::csr
// Provides: {"check_attribute_length"}
// Dependencies: {}
pub fn check_attribute_length < 'a > (values : asn1 :: SetOf < 'a , asn1 :: Tlv < 'a > > ,) -> Result < () , asn1 :: ParseError > { if values . count () != 1 { Err (asn1 :: ParseError :: new (asn1 :: ParseErrorKind :: InvalidValue)) } else { Ok (()) } }
};
}
