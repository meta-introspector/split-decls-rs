// Generated macro for warn_if_invalid_params (function)
macro_rules! Depcrate_x509_certificatewarn_if_invalid_params {
() => {
// Module: crate::x509::certificate
// Provides: {"warn_if_invalid_params"}
// Dependencies: {}
fn warn_if_invalid_params (py : pyo3 :: Python < '_ > , params : AlgorithmParameters < '_ > ,) -> pyo3 :: PyResult < () > { match params { AlgorithmParameters :: EcDsaWithSha224 (Some (..)) | AlgorithmParameters :: EcDsaWithSha256 (Some (..)) | AlgorithmParameters :: EcDsaWithSha384 (Some (..)) | AlgorithmParameters :: EcDsaWithSha512 (Some (..)) | AlgorithmParameters :: DsaWithSha224 (Some (..)) | AlgorithmParameters :: DsaWithSha256 (Some (..)) | AlgorithmParameters :: DsaWithSha384 (Some (..)) | AlgorithmParameters :: DsaWithSha512 (Some (..)) => { let warning_cls = types :: DEPRECATED_IN_41 . get (py) ? ; let message = c"The parsed certificate contains a NULL parameter value in its signature algorithm parameters. This is invalid and will be rejected in a future version of cryptography. If this certificate was created via Java, please upgrade to JDK21+ or the latest JDK11/17 once a fix is issued. If this certificate was created in some other fashion please report the issue to the cryptography issue tracker. See https://github.com/pyca/cryptography/issues/8996 and https://github.com/pyca/cryptography/issues/9253 for more details." ; pyo3 :: PyErr :: warn (py , & warning_cls , message , 2) ? ; } _ => { } } Ok (()) }
};
}
