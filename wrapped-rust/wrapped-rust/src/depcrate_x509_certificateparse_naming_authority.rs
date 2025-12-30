// Generated macro for parse_naming_authority (function)
macro_rules! Depcrate_x509_certificateparse_naming_authority {
() => {
// Module: crate::x509::certificate
// Provides: {"parse_naming_authority"}
// Dependencies: {}
fn parse_naming_authority < 'p > (py : pyo3 :: Python < 'p > , authority : NamingAuthority < '_ > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let py_id = match & authority . id { Some (data) => oid_to_py_oid (py , data) ? , None => py . None () . into_bound (py) , } ; let py_url = match authority . url { Some (data) => pyo3 :: types :: PyString :: new (py , data . as_str ()) . into_any () , None => py . None () . into_bound (py) , } ; let py_text = match authority . text { Some (data) => parse_display_text (py , data) ? , None => py . None () . into_bound (py) , } ; Ok (types :: NAMING_AUTHORITY . get (py) ? . call1 ((py_id , py_url , py_text)) ?) }
};
}
