// Generated macro for impl_49 (impl)
macro_rules! Depcrate_rustcrypto_implimpl_49 {
() => {
// Module: crate::rustcrypto_impl
// Provides: {"impl_49"}
// Dependencies: {}
impl < Rounds : Unsigned + Default > ChaChaAny < U24 , Rounds , X > { fn new (key : & GenericArray < u8 , U32 > , nonce : & GenericArray < u8 , U24 >) -> Self { ChaChaAny { state : Buffer { state : init_chacha_x (key , nonce , Rounds :: U32) , out : [0 ; BLOCK] , have : 0 , len : BIG_LEN , fresh : true , } , _nonce_size : Default :: default () , _rounds : Default :: default () , _is_x : Default :: default () , } } }
};
}
