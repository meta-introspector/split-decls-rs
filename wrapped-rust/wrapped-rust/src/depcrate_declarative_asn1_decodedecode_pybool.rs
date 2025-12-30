// Generated macro for decode_pybool (function)
macro_rules! Depcrate_declarative_asn1_decodedecode_pybool {
() => {
// Module: crate::declarative_asn1::decode
// Provides: {"decode_pybool"}
// Dependencies: {}
fn decode_pybool < 'a > (py : pyo3 :: Python < 'a > , parser : & mut Parser < 'a > , encoding : & Option < pyo3 :: Py < Encoding > > ,) -> ParseResult < pyo3 :: Bound < 'a , pyo3 :: types :: PyBool > > { let value = read_value :: < bool > (parser , encoding) ? ; Ok (pyo3 :: types :: PyBool :: new (py , value) . to_owned ()) }
};
}
