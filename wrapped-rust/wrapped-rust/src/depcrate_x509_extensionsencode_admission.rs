// Generated macro for encode_admission (function)
macro_rules! Depcrate_x509_extensionsencode_admission {
() => {
// Module: crate::x509::extensions
// Provides: {"encode_admission"}
// Dependencies: {}
fn encode_admission < 'a > (py : pyo3 :: Python < 'a > , ka_bytes : & 'a cryptography_keepalive :: KeepAlive < pyo3 :: pybacked :: PyBackedBytes > , ka_str : & 'a cryptography_keepalive :: KeepAlive < pyo3 :: pybacked :: PyBackedStr > , py_admission : & pyo3 :: Bound < 'a , pyo3 :: PyAny > ,) -> CryptographyResult < extensions :: Admission < 'a , Asn1Write > > { let py_admission_authority = py_admission . getattr (pyo3 :: intern ! (py , "admission_authority")) ? ; let admission_authority = if ! py_admission_authority . is_none () { Some (x509 :: common :: encode_general_name (py , ka_bytes , ka_str , & py_admission_authority ,) ?) } else { None } ; let py_naming_authority = py_admission . getattr (pyo3 :: intern ! (py , "naming_authority")) ? ; let naming_authority = if ! py_naming_authority . is_none () { Some (encode_naming_authority (py , ka_str , & py_naming_authority) ?) } else { None } ; let py_profession_infos = py_admission . getattr (pyo3 :: intern ! (py , "profession_infos")) ? ; let mut profession_infos = vec ! [] ; for py_info in py_profession_infos . try_iter () ? { profession_infos . push (encode_profession_info (py , ka_bytes , ka_str , & py_info ?) ?) ; } let profession_infos = asn1 :: SequenceOfWriter :: new (profession_infos) ; Ok (extensions :: Admission { admission_authority , naming_authority , profession_infos , }) }
};
}
