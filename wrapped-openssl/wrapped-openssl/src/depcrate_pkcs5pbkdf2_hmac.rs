// Generated macro for pbkdf2_hmac (function)
macro_rules! Depcrate_pkcs5pbkdf2_hmac {
() => {
// Module: crate::pkcs5
// Provides: {"pbkdf2_hmac"}
// Dependencies: {}
# [doc = " Derives a key from a password and salt using the PBKDF2-HMAC algorithm with a digest function."] # [corresponds (PKCS5_PBKDF2_HMAC)] pub fn pbkdf2_hmac (pass : & [u8] , salt : & [u8] , iter : usize , hash : MessageDigest , key : & mut [u8] ,) -> Result < () , ErrorStack > { unsafe { ffi :: init () ; cvt (ffi :: PKCS5_PBKDF2_HMAC (pass . as_ptr () as * const _ , pass . len () . try_into () . unwrap () , salt . as_ptr () , salt . len () . try_into () . unwrap () , iter . try_into () . unwrap () , hash . as_ptr () , key . len () . try_into () . unwrap () , key . as_mut_ptr () ,)) . map (| _ | ()) } }
};
}
