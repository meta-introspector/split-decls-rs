// Generated macro for impl_230 (impl)
macro_rules! Depcrate_signed_dataimpl_230 {
() => {
// Module: crate::signed_data
// Provides: {"impl_230"}
// Dependencies: {}
impl From < & Certificate > for SignerIdentifier { fn from (cert : & Certificate) -> Self { let tbs = cert . tbs_certificate () ; match tbs . get_extension :: < SubjectKeyIdentifier > () { Ok (Some ((_critical , ski))) => Self :: SubjectKeyIdentifier (ski) , _ => { let isn = IssuerAndSerialNumber { issuer : tbs . issuer () . clone () , serial_number : tbs . serial_number () . clone () , } ; Self :: IssuerAndSerialNumber (isn) } } } }
};
}
