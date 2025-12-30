// Generated macro for impl_51 (impl)
macro_rules! Depcrateimpl_51 {
() => {
// Module: crate
// Provides: {"impl_51"}
// Dependencies: {}
impl ClientCert { fn new (mut certkey : Credentials , meta : & Credential) -> Self { let Identity :: X509 (id) = & * certkey . identity else { panic ! ("only X.509 client certs supported") ; } ; let parsed_cert = webpki :: EndEntityCert :: try_from (match id . intermediates . last () { Some (root) => root , None => & id . end_entity , }) . unwrap () ; let issuer_dn = DistinguishedName :: in_sequence (parsed_cert . issuer ()) ; if let Some (scheme) = meta . use_signing_scheme { certkey . key = Box :: new (FixedSignatureSchemeSigningKey { key : certkey . key , scheme : lookup_scheme (scheme) , }) ; } Self { certkey , issuer_dn , must_match_issuer : meta . must_match_issuer , } } }
};
}
