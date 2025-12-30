// Generated macro for decode_pybytes (function)
macro_rules! Depcrate_declarative_asn1_decodedecode_pybytes {
() => {
// Module: crate::declarative_asn1::decode
// Provides: {"decode_pybytes"}
// Dependencies: {}
fn decode_pybytes < 'a > (py : pyo3 :: Python < 'a > , parser : & mut Parser < 'a > , encoding : & Option < pyo3 :: Py < Encoding > > ,) -> ParseResult < pyo3 :: Bound < 'a , pyo3 :: types :: PyBytes > > { let value = read_value :: < & [u8] > (parser , encoding) ? ; Ok (pyo3 :: types :: PyBytes :: new (py , value)) }
};
}
