// Generated macro for decode_generalized_time (function)
macro_rules! Depcrate_declarative_asn1_decodedecode_generalized_time {
() => {
// Module: crate::declarative_asn1::decode
// Provides: {"decode_generalized_time"}
// Dependencies: {}
fn decode_generalized_time < 'a > (py : pyo3 :: Python < 'a > , parser : & mut Parser < 'a > , encoding : & Option < pyo3 :: Py < Encoding > > ,) -> ParseResult < pyo3 :: Bound < 'a , GeneralizedTime > > { let value = read_value :: < asn1 :: GeneralizedTime > (parser , encoding) ? ; let dt = value . as_datetime () ; let microseconds = match value . nanoseconds () { Some (x) if x % 1_000 == 0 => x / 1_000 , Some (_) => { return Err (CryptographyError :: Py (pyo3 :: exceptions :: PyValueError :: new_err ("decoded GeneralizedTime data has higher precision than supported" . to_string () ,) ,)) } None => 0 , } ; let inner = crate :: x509 :: datetime_to_py_utc_with_microseconds (py , dt , microseconds) ? . cast_into :: < pyo3 :: types :: PyDateTime > () ? . unbind () ; Ok (pyo3 :: Bound :: new (py , GeneralizedTime { inner }) ?) }
};
}
