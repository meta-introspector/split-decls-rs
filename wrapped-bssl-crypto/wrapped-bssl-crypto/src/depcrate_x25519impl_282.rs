// Generated macro for impl_282 (impl)
macro_rules! Depcrate_x25519impl_282 {
() => {
// Module: crate::x25519
// Provides: {"impl_282"}
// Dependencies: {}
impl PrivateKey { # [doc = " Derive the shared key between this private key and a peer's public key."] # [doc = " Don't use the shared key directly, rather use a KDF and also include"] # [doc = " the two public values as inputs."] # [doc = ""] # [doc = " Will fail and produce `None` if the peer's public key is a point of"] # [doc = " small order. It is safe to react to this in non-constant time."] pub fn compute_shared_key (& self , other_public_key : & PublicKey) -> Option < [u8 ; SHARED_KEY_LEN] > { unsafe { with_output_array_fallible (| out , _ | { bssl_sys :: X25519 (out , self . 0 . as_ffi_ptr () , other_public_key . as_ffi_ptr ()) == 1 }) } } # [doc = " Generate a new key pair."] pub fn generate () -> (PublicKey , PrivateKey) { let mut public_key_uninit = core :: mem :: MaybeUninit :: < [u8 ; PUBLIC_KEY_LEN] > :: uninit () ; let mut private_key_uninit = core :: mem :: MaybeUninit :: < [u8 ; PRIVATE_KEY_LEN] > :: uninit () ; unsafe { bssl_sys :: X25519_keypair (public_key_uninit . as_mut_ptr () as * mut u8 , private_key_uninit . as_mut_ptr () as * mut u8 ,) ; (public_key_uninit . assume_init () , PrivateKey (private_key_uninit . assume_init ()) ,) } } # [doc = " Compute the public key corresponding to this private key."] pub fn to_public (& self) -> PublicKey { unsafe { with_output_array (| out , _ | { bssl_sys :: X25519_public_from_private (out , self . 0 . as_ffi_ptr ()) ; }) } } }
};
}
