// Generated macro for Store (struct)
macro_rules! Depcrate_trust_storeStore {
() => {
// Module: crate::trust_store
// Provides: {"Store"}
// Dependencies: {}
# [doc = " A `Store` represents the core state needed for X.509 path validation."] pub struct Store < 'a , B : CryptoOps > { by_subject : HashMap < Name < 'a > , Vec < VerificationCertificate < 'a , B > > > , }
};
}
