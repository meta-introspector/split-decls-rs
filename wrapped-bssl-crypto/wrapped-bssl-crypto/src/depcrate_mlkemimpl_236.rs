// Generated macro for impl_236 (impl)
macro_rules! Depcrate_mlkemimpl_236 {
() => {
// Module: crate::mlkem
// Provides: {"impl_236"}
// Dependencies: {}
impl PublicKey768 { # [doc = " Parse a public key from NIST's defined format."] pub fn parse (encoded : & [u8]) -> Option < Self > { let mut cbs = as_cbs (encoded) ; unsafe { initialized_boxed_struct_fallible (| pub_key | { bssl_sys :: MLKEM768_parse_public_key (pub_key , & mut cbs) == 1 && cbs . len == 0 }) } . map (Self) } # [doc = " Return the serialization of this public key."] pub fn to_bytes (& self) -> Vec < u8 > { unsafe { cbb_to_vec (PUBLIC_KEY_BYTES_768 , | cbb | { let ok = bssl_sys :: MLKEM768_marshal_public_key (cbb , & * self . 0) ; assert_eq ! (ok , 1) ; }) } } # [doc = " Generate a secret key and encrypt it to this public key, returning the"] # [doc = " ciphertext and the shared secret key."] pub fn encapsulate (& self) -> (Vec < u8 > , [u8 ; SHARED_SECRET_BYTES]) { let mut ciphertext = Box :: new_uninit_slice (CIPHERTEXT_BYTES_768) ; let mut shared_secret = MaybeUninit :: < [u8 ; SHARED_SECRET_BYTES] > :: uninit () ; unsafe { bssl_sys :: MLKEM768_encap (ciphertext . as_mut_ptr () as * mut u8 , shared_secret . as_mut_ptr () as * mut u8 , & * self . 0 ,) ; (ciphertext . assume_init () . into () , shared_secret . assume_init ()) } } }
};
}
