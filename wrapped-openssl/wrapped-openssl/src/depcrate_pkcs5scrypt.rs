// Generated macro for scrypt (function)
macro_rules! Depcrate_pkcs5scrypt {
() => {
// Module: crate::pkcs5
// Provides: {"scrypt"}
// Dependencies: {}
# [doc = " Derives a key from a password and salt using the scrypt algorithm."] # [doc = ""] # [doc = " Requires OpenSSL 1.1.0 or newer."] # [corresponds (EVP_PBE_scrypt)] # [cfg (all (any (ossl110 , boringssl , awslc) , not (osslconf = "OPENSSL_NO_SCRYPT")))] # [allow (clippy :: useless_conversion)] pub fn scrypt (pass : & [u8] , salt : & [u8] , n : u64 , r : u64 , p : u64 , maxmem : u64 , key : & mut [u8] ,) -> Result < () , ErrorStack > { unsafe { ffi :: init () ; cvt (ffi :: EVP_PBE_scrypt (pass . as_ptr () as * const _ , pass . len () , salt . as_ptr () as * const _ , salt . len () , n , r , p , maxmem . try_into () . unwrap () , key . as_mut_ptr () as * mut _ , key . len () ,)) . map (| _ | ()) } }
};
}
