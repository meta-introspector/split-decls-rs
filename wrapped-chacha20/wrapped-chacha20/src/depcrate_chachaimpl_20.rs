// Generated macro for impl_20 (impl)
macro_rules! Depcrate_chachaimpl_20 {
() => {
// Module: crate::chacha
// Provides: {"impl_20"}
// Dependencies: {}
impl < R : Rounds > KeyIvInit for ChaChaCore < R , Ietf > { # [inline] fn new (key : & Key , iv : & Nonce) -> Self { ChaChaCore :: < R , Ietf > :: new (key . as_ref () , iv . as_ref ()) } }
};
}
