// Generated macro for private_key_from_pem (macro)
macro_rules! Depcrate_macrosprivate_key_from_pem {
() => {
// Module: crate::macros
// Provides: {"private_key_from_pem"}
// Dependencies: {}
macro_rules ! private_key_from_pem { ($ (# [$ m : meta]) * $ n : ident , $ (# [$ m2 : meta]) * $ n2 : ident , $ (# [$ m3 : meta]) * $ n3 : ident , $ t : ty , $ f : path) => { from_pem ! ($ (# [$ m]) * $ n , $ t , $ f) ; $ (# [$ m2]) * pub fn $ n2 (pem : & [u8] , passphrase : & [u8]) -> Result <$ t , crate :: error :: ErrorStack > { unsafe { ffi :: init () ; let bio = crate :: bio :: MemBioSlice :: new (pem) ?; let passphrase = :: std :: ffi :: CString :: new (passphrase) . unwrap () ; crate :: cvt_p ($ f (bio . as_ptr () , :: std :: ptr :: null_mut () , None , passphrase . as_ptr () as * const _ as * mut _)) . map (| p | :: foreign_types :: ForeignType :: from_ptr (p)) } } $ (# [$ m3]) * pub fn $ n3 < F > (pem : & [u8] , callback : F) -> Result <$ t , crate :: error :: ErrorStack > where F : FnOnce (& mut [u8]) -> Result < usize , crate :: error :: ErrorStack > { unsafe { ffi :: init () ; let mut cb = crate :: util :: CallbackState :: new (callback) ; let bio = crate :: bio :: MemBioSlice :: new (pem) ?; crate :: cvt_p ($ f (bio . as_ptr () , :: std :: ptr :: null_mut () , Some (crate :: util :: invoke_passwd_cb ::< F >) , & mut cb as * mut _ as * mut _)) . map (| p | :: foreign_types :: ForeignType :: from_ptr (p)) } } } }
};
}
