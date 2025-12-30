// Generated macro for PyPolicy (struct)
macro_rules! Depcrate_x509_verify_policyPyPolicy {
() => {
// Module: crate::x509::verify::policy
// Provides: {"PyPolicy"}
// Dependencies: {}
# [doc = " Python-accessible wrapper for a cryptography_x509_verification::policy::Policy."] # [pyo3 :: pyclass (module = "cryptography.x509.verification" , name = "Policy" , frozen)] pub (crate) struct PyPolicy { pub (super) policy_definition : OwnedPolicyDefinition , pub (super) subject : pyo3 :: Py < pyo3 :: PyAny > , }
};
}
