// Generated macro for decode_pyint (function)
macro_rules! Depcrate_declarative_asn1_decodedecode_pyint {
() => {
// Module: crate::declarative_asn1::decode
// Provides: {"decode_pyint"}
// Dependencies: {}
fn decode_pyint < 'a > (py : pyo3 :: Python < 'a > , parser : & mut Parser < 'a > , encoding : & Option < pyo3 :: Py < Encoding > > ,) -> ParseResult < pyo3 :: Bound < 'a , pyo3 :: types :: PyInt > > { let value = read_value :: < asn1 :: BigInt < 'a > > (parser , encoding) ? ; let pyint = big_byte_slice_to_py_int (py , value . as_bytes ()) ? . cast_into :: < pyo3 :: types :: PyInt > () ? ; Ok (pyint) }
};
}
