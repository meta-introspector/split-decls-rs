// Generated macro for decode_printable_string (function)
macro_rules! Depcrate_declarative_asn1_decodedecode_printable_string {
() => {
// Module: crate::declarative_asn1::decode
// Provides: {"decode_printable_string"}
// Dependencies: {}
fn decode_printable_string < 'a > (py : pyo3 :: Python < 'a > , parser : & mut Parser < 'a > , encoding : & Option < pyo3 :: Py < Encoding > > ,) -> ParseResult < pyo3 :: Bound < 'a , PrintableString > > { let value = read_value :: < asn1 :: PrintableString < 'a > > (parser , encoding) ? . as_str () ; let inner = pyo3 :: types :: PyString :: new (py , value) . unbind () ; Ok (pyo3 :: Bound :: new (py , PrintableString { inner }) ?) }
};
}
