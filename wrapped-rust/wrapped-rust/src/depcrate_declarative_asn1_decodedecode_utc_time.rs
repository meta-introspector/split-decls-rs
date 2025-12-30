// Generated macro for decode_utc_time (function)
macro_rules! Depcrate_declarative_asn1_decodedecode_utc_time {
() => {
// Module: crate::declarative_asn1::decode
// Provides: {"decode_utc_time"}
// Dependencies: {}
fn decode_utc_time < 'a > (py : pyo3 :: Python < 'a > , parser : & mut Parser < 'a > , encoding : & Option < pyo3 :: Py < Encoding > > ,) -> ParseResult < pyo3 :: Bound < 'a , UtcTime > > { let value = read_value :: < asn1 :: UtcTime > (parser , encoding) ? ; let dt = value . as_datetime () ; let inner = crate :: x509 :: datetime_to_py_utc (py , dt) ? . cast_into :: < pyo3 :: types :: PyDateTime > () ? . unbind () ; Ok (pyo3 :: Bound :: new (py , UtcTime { inner }) ?) }
};
}
