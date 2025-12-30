// Generated macro for decode_pystr (function)
macro_rules! Depcrate_declarative_asn1_decodedecode_pystr {
() => {
// Module: crate::declarative_asn1::decode
// Provides: {"decode_pystr"}
// Dependencies: {}
fn decode_pystr < 'a > (py : pyo3 :: Python < 'a > , parser : & mut Parser < 'a > , encoding : & Option < pyo3 :: Py < Encoding > > ,) -> ParseResult < pyo3 :: Bound < 'a , pyo3 :: types :: PyString > > { let value = read_value :: < asn1 :: Utf8String < 'a > > (parser , encoding) ? ; Ok (pyo3 :: types :: PyString :: new (py , value . as_str ())) }
};
}
