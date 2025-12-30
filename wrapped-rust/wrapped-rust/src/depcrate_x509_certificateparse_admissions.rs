// Generated macro for parse_admissions (function)
macro_rules! Depcrate_x509_certificateparse_admissions {
() => {
// Module: crate::x509::certificate
// Provides: {"parse_admissions"}
// Dependencies: {}
fn parse_admissions < 'p , 'a > (py : pyo3 :: Python < 'p > , admissions : & asn1 :: SequenceOf < 'a , Admission < 'a , Asn1Read > > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let py_admissions = pyo3 :: types :: PyList :: empty (py) ; for admission in admissions . clone () { let py_admission_authority = match admission . admission_authority { Some (authority) => x509 :: parse_general_name (py , authority) ? , None => py . None () . into_bound (py) , } ; let py_naming_authority = match admission . naming_authority { Some (data) => parse_naming_authority (py , data) ? , None => py . None () . into_bound (py) , } ; let py_infos = parse_profession_infos (py , & admission . profession_infos) ? ; let py_entry = types :: ADMISSION . get (py) ? . call1 ((py_admission_authority , py_naming_authority , py_infos ,)) ? ; py_admissions . append (py_entry) ? ; } Ok (py_admissions . into_any ()) }
};
}
