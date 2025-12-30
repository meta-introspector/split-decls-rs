// Generated macro for impl_709 (impl)
macro_rules! Depcrate_pkeyimpl_709 {
() => {
// Module: crate::pkey
// Provides: {"impl_709"}
// Dependencies: {}
impl < T > PKeyRef < T > { # [doc = " Returns a copy of the internal RSA key."] # [corresponds (EVP_PKEY_get1_RSA)] pub fn rsa (& self) -> Result < Rsa < T > , ErrorStack > { unsafe { let rsa = cvt_p (ffi :: EVP_PKEY_get1_RSA (self . as_ptr ())) ? ; Ok (Rsa :: from_ptr (rsa)) } } # [doc = " Returns a copy of the internal DSA key."] # [corresponds (EVP_PKEY_get1_DSA)] pub fn dsa (& self) -> Result < Dsa < T > , ErrorStack > { unsafe { let dsa = cvt_p (ffi :: EVP_PKEY_get1_DSA (self . as_ptr ())) ? ; Ok (Dsa :: from_ptr (dsa)) } } # [doc = " Returns a copy of the internal DH key."] # [corresponds (EVP_PKEY_get1_DH)] pub fn dh (& self) -> Result < Dh < T > , ErrorStack > { unsafe { let dh = cvt_p (ffi :: EVP_PKEY_get1_DH (self . as_ptr ())) ? ; Ok (Dh :: from_ptr (dh)) } } # [doc = " Returns a copy of the internal elliptic curve key."] # [corresponds (EVP_PKEY_get1_EC_KEY)] pub fn ec_key (& self) -> Result < EcKey < T > , ErrorStack > { unsafe { let ec_key = cvt_p (ffi :: EVP_PKEY_get1_EC_KEY (self . as_ptr ())) ? ; Ok (EcKey :: from_ptr (ec_key)) } } # [doc = " Returns the `Id` that represents the type of this key."] # [corresponds (EVP_PKEY_id)] pub fn id (& self) -> Id { unsafe { Id :: from_raw (ffi :: EVP_PKEY_id (self . as_ptr ())) } } # [doc = " Returns the maximum size of a signature in bytes."] # [corresponds (EVP_PKEY_size)] pub fn size (& self) -> usize { unsafe { ffi :: EVP_PKEY_size (self . as_ptr ()) as usize } } }
};
}
