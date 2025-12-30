// Generated macro for tests (module)
macro_rules! Depcrate_trust_storetests {
() => {
// Module: crate::trust_store
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Store ; use crate :: certificate :: tests :: PublicKeyErrorOps ; use crate :: ops :: tests :: { cert , v1_cert_pem } ; use crate :: VerificationCertificate ; # [test] fn test_store () { let cert_pem = v1_cert_pem () ; let c1 = cert (& cert_pem) ; let c2 = cert (& cert_pem) ; let cert1 = VerificationCertificate :: new (& c1 , ()) ; let cert2 = VerificationCertificate :: new (& c2 , ()) ; let store = Store :: < '_ , PublicKeyErrorOps > :: new ([cert1]) ; assert ! (store . contains (& cert2)) ; } }
};
}
