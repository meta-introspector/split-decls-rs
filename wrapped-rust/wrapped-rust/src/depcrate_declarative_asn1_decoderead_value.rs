// Generated macro for read_value (function)
macro_rules! Depcrate_declarative_asn1_decoderead_value {
() => {
// Module: crate::declarative_asn1::decode
// Provides: {"read_value"}
// Dependencies: {}
fn read_value < 'a , T : asn1 :: SimpleAsn1Readable < 'a > > (parser : & mut Parser < 'a > , encoding : & Option < pyo3 :: Py < Encoding > > ,) -> ParseResult < T > { let value = match encoding { Some (e) => match e . get () { Encoding :: Implicit (n) => parser . read_implicit_element :: < T > (* n) , Encoding :: Explicit (n) => parser . read_explicit_element :: < T > (* n) , } , None => parser . read_element :: < T > () , } ? ; Ok (value) }
};
}
