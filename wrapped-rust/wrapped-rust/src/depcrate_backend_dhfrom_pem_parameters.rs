// Generated macro for from_pem_parameters (function)
macro_rules! Depcrate_backend_dhfrom_pem_parameters {
() => {
// Module: crate::backend::dh
// Provides: {"from_pem_parameters"}
// Dependencies: {}
# [pyo3 :: pyfunction] # [pyo3 (signature = (data , backend = None))] fn from_pem_parameters (data : & [u8] , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < DHParameters > { let _ = backend ; let parsed = x509 :: find_in_pem (data , | p | p . tag () == "DH PARAMETERS" || p . tag () == "X9.42 DH PARAMETERS" , "Valid PEM but no BEGIN DH PARAMETERS/END DH PARAMETERS delimiters. Are you sure this is a DH parameters?" ,) ? ; from_der_parameters (parsed . contents () , None) }
};
}
