// Generated macro for map_crl_error (function)
macro_rules! Depcrate_errormap_crl_error {
() => {
// Module: crate::error
// Provides: {"map_crl_error"}
// Dependencies: {}
fn map_crl_error (err : CertRevocationListError) -> rustls_result { use rustls_result :: * ; match err { CertRevocationListError :: BadSignature => CertRevocationListBadSignature , CertRevocationListError :: UnsupportedSignatureAlgorithmContext { .. } => { CertRevocationListUnsupportedSignatureAlgorithm } CertRevocationListError :: InvalidCrlNumber => CertRevocationListInvalidCrlNumber , CertRevocationListError :: InvalidRevokedCertSerialNumber => { CertRevocationListInvalidRevokedCertSerialNumber } CertRevocationListError :: IssuerInvalidForCrl => CertRevocationListIssuerInvalidForCrl , CertRevocationListError :: Other (_) => CertRevocationListOtherError , CertRevocationListError :: ParseError => CertRevocationListParseError , CertRevocationListError :: UnsupportedCrlVersion => CertRevocationListUnsupportedCrlVersion , CertRevocationListError :: UnsupportedCriticalExtension => { CertRevocationListUnsupportedCriticalExtension } CertRevocationListError :: UnsupportedDeltaCrl => CertRevocationListUnsupportedDeltaCrl , CertRevocationListError :: UnsupportedIndirectCrl => CertRevocationListUnsupportedIndirectCrl , CertRevocationListError :: UnsupportedRevocationReason => { CertRevocationListUnsupportedRevocationReason } _ => CertRevocationListOtherError , } }
};
}
