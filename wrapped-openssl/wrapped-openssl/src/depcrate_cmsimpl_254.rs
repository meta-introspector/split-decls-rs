// Generated macro for impl_254 (impl)
macro_rules! Depcrate_cmsimpl_254 {
() => {
// Module: crate::cms
// Provides: {"impl_254"}
// Dependencies: {}
impl CmsContentInfoRef { # [doc = " Given the sender's private key, `pkey` and the recipient's certificate, `cert`,"] # [doc = " decrypt the data in `self`."] # [corresponds (CMS_decrypt)] pub fn decrypt < T > (& self , pkey : & PKeyRef < T > , cert : & X509) -> Result < Vec < u8 > , ErrorStack > where T : HasPrivate , { unsafe { let pkey = pkey . as_ptr () ; let cert = cert . as_ptr () ; let out = MemBio :: new () ? ; cvt (ffi :: CMS_decrypt (self . as_ptr () , pkey , cert , ptr :: null_mut () , out . as_ptr () , 0 ,)) ? ; Ok (out . get_buf () . to_owned ()) } } # [doc = " Given the sender's private key, `pkey`,"] # [doc = " decrypt the data in `self` without validating the recipient certificate."] # [doc = ""] # [doc = " *Warning*: Not checking the recipient certificate may leave you vulnerable to Bleichenbacher's attack on PKCS#1 v1.5 RSA padding."] # [corresponds (CMS_decrypt)] pub fn decrypt_without_cert_check < T > (& self , pkey : & PKeyRef < T >) -> Result < Vec < u8 > , ErrorStack > where T : HasPrivate , { unsafe { let pkey = pkey . as_ptr () ; let out = MemBio :: new () ? ; cvt (ffi :: CMS_decrypt (self . as_ptr () , pkey , ptr :: null_mut () , ptr :: null_mut () , out . as_ptr () , 0 ,)) ? ; Ok (out . get_buf () . to_owned ()) } } to_der ! { # [doc = " Serializes this CmsContentInfo using DER."] # [corresponds (i2d_CMS_ContentInfo)] to_der , ffi :: i2d_CMS_ContentInfo } to_pem ! { # [doc = " Serializes this CmsContentInfo using DER."] # [corresponds (PEM_write_bio_CMS)] to_pem , ffi :: PEM_write_bio_CMS } }
};
}
