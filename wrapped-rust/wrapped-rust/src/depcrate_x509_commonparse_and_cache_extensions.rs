// Generated macro for parse_and_cache_extensions (function)
macro_rules! Depcrate_x509_commonparse_and_cache_extensions {
() => {
// Module: crate::x509::common
// Provides: {"parse_and_cache_extensions"}
// Dependencies: {}
pub (crate) fn parse_and_cache_extensions < 'p , F : Fn (& Extension < 'p >) -> Result < Option < pyo3 :: Bound < 'p , pyo3 :: PyAny > > , CryptographyError > , > (py : pyo3 :: Python < 'p > , cached_extensions : & pyo3 :: sync :: PyOnceLock < pyo3 :: Py < pyo3 :: PyAny > > , raw_extensions : & Option < RawExtensions < 'p > > , parse_ext : F ,) -> pyo3 :: PyResult < pyo3 :: Py < pyo3 :: PyAny > > { cached_extensions . get_or_try_init (py , | | { let extensions = match Extensions :: from_raw_extensions (raw_extensions . as_ref ()) { Ok (extensions) => extensions , Err (DuplicateExtensionsError (oid)) => { let oid_obj = oid_to_py_oid (py , & oid) ? ; return Err (exceptions :: DuplicateExtension :: new_err ((format ! ("Duplicate {} extension found" , & oid) , oid_obj . unbind () ,))) ; } } ; let exts = pyo3 :: types :: PyList :: empty (py) ; for raw_ext in extensions . iter () { let oid_obj = oid_to_py_oid (py , & raw_ext . extn_id) ? ; let extn_value = match parse_ext (& raw_ext) ? { Some (e) => e , None => types :: UNRECOGNIZED_EXTENSION . get (py) ? . call1 ((oid_obj . clone () , raw_ext . extn_value)) ? , } ; let ext_obj = types :: EXTENSION . get (py) ? . call1 ((oid_obj , raw_ext . critical , extn_value)) ? ; exts . append (ext_obj) ? ; } Ok (types :: EXTENSIONS . get (py) ? . call1 ((exts ,)) ? . unbind ()) }) . map (| p | p . clone_ref (py)) }
};
}
