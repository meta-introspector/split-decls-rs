// Generated macro for impl_474 (impl)
macro_rules! Depcrate_errorimpl_474 {
() => {
// Module: crate::error
// Provides: {"impl_474"}
// Dependencies: {}
impl CryptographyError { pub (crate) fn add_location (self , loc : asn1 :: ParseLocation) -> Self { match self { CryptographyError :: Py (e) => CryptographyError :: Py (e) , CryptographyError :: Asn1Parse (e) => CryptographyError :: Asn1Parse (e . add_location (loc)) , CryptographyError :: KeyParsing (e) => CryptographyError :: KeyParsing (e . add_location (loc)) , CryptographyError :: Asn1Write (e) => CryptographyError :: Asn1Write (e) , CryptographyError :: OpenSSL (e) => CryptographyError :: OpenSSL (e) , } } pub (crate) fn add_note (self , py : pyo3 :: Python < '_ > , note : & str) -> Self { let pyerr : pyo3 :: PyErr = self . into () ; # [cfg (Py_3_11)] { _ = pyerr . add_note (py , note) ; } # [cfg (not (Py_3_11))] { _ = py ; _ = note ; } Self :: Py (pyerr) } }
};
}
