// Generated macro for pkcs7_mod (module)
macro_rules! Depcrate_pkcs7pkcs7_mod {
() => {
// Module: crate::pkcs7
// Provides: {"pkcs7_mod"}
// Dependencies: {}
# [pyo3 :: pymodule (gil_used = false)] # [pyo3 (name = "pkcs7")] pub (crate) mod pkcs7_mod { # [pymodule_export] use super :: { decrypt_der , decrypt_pem , decrypt_smime , encrypt_and_serialize , load_der_pkcs7_certificates , load_pem_pkcs7_certificates , serialize_certificates , sign_and_serialize , } ; }
};
}
