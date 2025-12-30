// Generated macro for impl_138 (impl)
macro_rules! Depcrate_content_infoimpl_138 {
() => {
// Module: crate::content_info
// Provides: {"impl_138"}
// Dependencies: {}
# [doc = " Convert a vector of Certificates to a certs-only SignedData message"] impl TryFrom < PkiPath > for ContentInfo { type Error = der :: Error ; fn try_from (pki_path : PkiPath) -> der :: Result < Self > { let mut certs = CertificateSet (Default :: default ()) ; for cert in pki_path { certs . 0 . insert (CertificateChoices :: Certificate (cert)) ? ; } let sd = SignedData { version : CmsVersion :: V1 , digest_algorithms : SetOfVec :: default () , encap_content_info : EncapsulatedContentInfo { econtent_type : const_oid :: db :: rfc5911 :: ID_DATA , econtent : None , } , certificates : Some (certs) , crls : Some (RevocationInfoChoices (Default :: default ())) , signer_infos : SignerInfos (Default :: default ()) , } ; let signed_data = sd . to_der () ? ; let content = AnyRef :: try_from (signed_data . as_slice ()) ? ; Ok (ContentInfo { content_type : const_oid :: db :: rfc5911 :: ID_SIGNED_DATA , content : Any :: from (content) , }) } }
};
}
