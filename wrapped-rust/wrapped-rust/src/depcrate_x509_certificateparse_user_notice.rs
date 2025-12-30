// Generated macro for parse_user_notice (function)
macro_rules! Depcrate_x509_certificateparse_user_notice {
() => {
// Module: crate::x509::certificate
// Provides: {"parse_user_notice"}
// Dependencies: {}
fn parse_user_notice < 'p > (py : pyo3 :: Python < 'p > , un : UserNotice < '_ , Asn1Read > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let et = match un . explicit_text { Some (data) => parse_display_text (py , data) ? , None => py . None () . into_bound (py) , } ; let nr = match un . notice_ref { Some (data) => { let org = parse_display_text (py , data . organization) ? ; let numbers = pyo3 :: types :: PyList :: empty (py) ; for num in data . notice_numbers . clone () { numbers . append (big_byte_slice_to_py_int (py , num . as_bytes ()) ?) ? ; } types :: NOTICE_REFERENCE . get (py) ? . call1 ((org , numbers)) ? } None => py . None () . into_bound (py) , } ; Ok (types :: USER_NOTICE . get (py) ? . call1 ((nr , et)) ?) }
};
}
