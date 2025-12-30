// Generated macro for impl_53 (impl)
macro_rules! Depcrate_rustcrypto_implimpl_53 {
() => {
// Module: crate::rustcrypto_impl
// Provides: {"impl_53"}
// Dependencies: {}
impl < Rounds : Unsigned + Default > NewCipher for ChaChaAny < U24 , Rounds , X > { type KeySize = U32 ; type NonceSize = U24 ; # [inline] fn new (key : & GenericArray < u8 , Self :: KeySize > , nonce : & GenericArray < u8 , Self :: NonceSize > ,) -> Self { Self :: new (key , nonce) } }
};
}
