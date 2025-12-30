// Generated macro for PyCriticality (enum)
macro_rules! Depcrate_x509_verify_extension_policyPyCriticality {
() => {
// Module: crate::x509::verify::extension_policy
// Provides: {"PyCriticality"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , eq , module = "cryptography.x509.verification" , name = "Criticality")] # [derive (PartialEq , Eq , Clone)] pub (crate) enum PyCriticality { # [pyo3 (name = "CRITICAL")] Critical , # [pyo3 (name = "AGNOSTIC")] Agnostic , # [pyo3 (name = "NON_CRITICAL")] NonCritical , }
};
}
