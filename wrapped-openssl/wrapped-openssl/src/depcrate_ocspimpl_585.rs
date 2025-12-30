// Generated macro for impl_585 (impl)
macro_rules! Depcrate_ocspimpl_585 {
() => {
// Module: crate::ocsp
// Provides: {"impl_585"}
// Dependencies: {}
impl OcspCertId { # [doc = " Constructs a certificate ID for certificate `subject`."] # [corresponds (OCSP_cert_to_id)] pub fn from_cert (digest : MessageDigest , subject : & X509Ref , issuer : & X509Ref ,) -> Result < OcspCertId , ErrorStack > { unsafe { cvt_p (ffi :: OCSP_cert_to_id (digest . as_ptr () , subject . as_ptr () , issuer . as_ptr () ,)) . map (OcspCertId) } } }
};
}
