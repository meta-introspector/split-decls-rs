// Generated macro for impl_48 (impl)
macro_rules! Depcrate_rustcrypto_implimpl_48 {
() => {
// Module: crate::rustcrypto_impl
// Provides: {"impl_48"}
// Dependencies: {}
impl < NonceSize , Rounds > ChaChaAny < NonceSize , Rounds , O > where NonceSize : Unsigned + ArrayLength < u8 > + Default , Rounds : Default , { # [inline] fn new (key : & GenericArray < u8 , U32 > , nonce : & GenericArray < u8 , NonceSize >) -> Self { let nonce_len = nonce . len () ; ChaChaAny { state : Buffer { state : init_chacha (key , nonce) , out : [0 ; BLOCK] , have : 0 , len : if nonce_len == 12 { SMALL_LEN } else { BIG_LEN } , fresh : nonce_len != 12 , } , _nonce_size : Default :: default () , _rounds : Default :: default () , _is_x : Default :: default () , } } }
};
}
