// Generated macro for impl_82 (impl)
macro_rules! Depcrate_trust_storeimpl_82 {
() => {
// Module: crate::trust_store
// Provides: {"impl_82"}
// Dependencies: {}
impl < 'a , B : CryptoOps > Store < 'a , B > { # [doc = " Create a new `Store` from the given iterable certificate source."] pub fn new (trusted : impl IntoIterator < Item = VerificationCertificate < 'a , B > >) -> Self { let mut by_subject : HashMap < Name < 'a > , Vec < VerificationCertificate < 'a , B > > > = HashMap :: new () ; for cert in trusted { by_subject . entry (cert . certificate () . tbs_cert . subject . clone ()) . or_default () . push (cert) ; } Store { by_subject } } # [doc = " Returns whether this store contains the given certificate."] pub fn contains (& self , cert : & VerificationCertificate < 'a , B >) -> bool { self . get_by_subject (& cert . certificate () . tbs_cert . subject) . contains (cert) } pub fn get_by_subject (& self , subject : & Name < 'a >) -> & [VerificationCertificate < 'a , B >] { self . by_subject . get (subject) . map (| v | v . as_slice ()) . unwrap_or_default () } }
};
}
