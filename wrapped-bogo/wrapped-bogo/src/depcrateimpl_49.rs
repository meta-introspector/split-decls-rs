// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
impl client :: ClientCredentialResolver for MultipleClientCredentialResolver { fn resolve (& self , request : & CredentialRequest < '_ >) -> Option < SelectedCredential > { let sig_schemes = request . signature_schemes () ; let root_hint_subjects = request . root_hint_subjects () ; for sig_scheme in sig_schemes . iter () . copied () { for (i , cert) in self . additional . iter () . enumerate () { if cert . must_match_issuer && ! root_hint_subjects . iter () . any (| dn | dn . as_ref () == cert . issuer_dn . as_ref ()) { continue ; } if let Some (signer) = cert . certkey . signer (& [sig_scheme]) { assert ! (Some (i as isize) == self . expect_selected || self . expect_selected . is_none ()) ; return Some (signer) ; } } } if let Some (cert) = & self . default { if let Some (signer) = cert . certkey . signer (sig_schemes) { assert ! (matches ! (self . expect_selected , Some (- 1) | None)) ; return Some (signer) ; } } assert_eq ! (self . expect_selected , None) ; let all_must_match_issuer = self . additional . iter () . chain (self . default . iter ()) . all (| item | item . must_match_issuer) ; quit (match all_must_match_issuer { true => ":NO_MATCHING_ISSUER:" , false => ":NO_COMMON_SIGNATURE_ALGORITHMS:" , }) } fn supported_certificate_types (& self) -> & 'static [CertificateType] { match self . default . is_some () || ! self . additional . is_empty () { true => & [CertificateType :: X509] , false => & [] , } } }
};
}
