// Generated macro for PyExtensionPolicy (struct)
macro_rules! Depcrate_x509_verify_extension_policyPyExtensionPolicy {
() => {
// Module: crate::x509::verify::extension_policy
// Provides: {"PyExtensionPolicy"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.x509.verification" , name = "ExtensionPolicy")] pub (crate) struct PyExtensionPolicy { inner_policy : ExtensionPolicy < 'static , PyCryptoOps > , already_set_oids : HashSet < asn1 :: ObjectIdentifier > , }
};
}
