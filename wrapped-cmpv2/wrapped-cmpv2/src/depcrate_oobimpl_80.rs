// Generated macro for impl_80 (impl)
macro_rules! Depcrate_oobimpl_80 {
() => {
// Module: crate::oob
// Provides: {"impl_80"}
// Dependencies: {}
# [cfg (feature = "digest")] impl < P > OobCertHash < P > where P : Profile , { # [doc = " Create an [`OobCertHash`] from a given certificate"] pub fn from_certificate < D > (cert : & CertificateInner < P >) -> der :: Result < Self > where D : digest :: Digest + AssociatedOid , { Ok (Self { hash_alg : Some (AlgorithmIdentifierOwned { oid : D :: OID , parameters : Some (Null . into ()) , }) , cert_id : Some (CertId { issuer : GeneralName :: DirectoryName (cert . tbs_certificate () . issuer () . clone ()) , serial_number : cert . tbs_certificate () . serial_number () . clone () , }) , hash_val : BitString :: from_bytes (& cert . hash :: < D > () ?) ? , }) } }
};
}
