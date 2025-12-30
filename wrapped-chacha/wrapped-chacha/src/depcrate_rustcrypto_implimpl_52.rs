// Generated macro for impl_52 (impl)
macro_rules! Depcrate_rustcrypto_implimpl_52 {
() => {
// Module: crate::rustcrypto_impl
// Provides: {"impl_52"}
// Dependencies: {}
impl < NonceSize , Rounds > NewCipher for ChaChaAny < NonceSize , Rounds , O > where NonceSize : Unsigned + ArrayLength < u8 > + Default , Rounds : Default , { type KeySize = U32 ; type NonceSize = NonceSize ; # [inline] fn new (key : & GenericArray < u8 , Self :: KeySize > , nonce : & GenericArray < u8 , Self :: NonceSize > ,) -> Self { Self :: new (key , nonce) } }
};
}
