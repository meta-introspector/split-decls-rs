// Generated macro for PolicyBuilder (struct)
macro_rules! Depcrate_x509_verifyPolicyBuilder {
() => {
// Module: crate::x509::verify
// Provides: {"PolicyBuilder"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.x509.verification")] pub (crate) struct PolicyBuilder { time : Option < asn1 :: DateTime > , store : Option < pyo3 :: Py < PyStore > > , max_chain_depth : Option < u8 > , ca_ext_policy : Option < pyo3 :: Py < PyExtensionPolicy > > , ee_ext_policy : Option < pyo3 :: Py < PyExtensionPolicy > > , }
};
}
