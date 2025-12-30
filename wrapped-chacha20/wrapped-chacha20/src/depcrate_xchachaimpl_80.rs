// Generated macro for impl_80 (impl)
macro_rules! Depcrate_xchachaimpl_80 {
() => {
// Module: crate::xchacha
// Provides: {"impl_80"}
// Dependencies: {}
impl < R : Rounds > KeyIvInit for XChaChaCore < R > { fn new (key : & Key , iv : & XNonce) -> Self { let subkey = hchacha :: < R > (key , iv [.. 16] . as_ref () . try_into () . unwrap ()) ; let mut nonce = [0u8 ; 12] ; nonce [4 ..] . copy_from_slice (& iv [16 ..]) ; Self (ChaChaCore :: < R , Ietf > :: new (subkey . as_ref () , & nonce)) } }
};
}
