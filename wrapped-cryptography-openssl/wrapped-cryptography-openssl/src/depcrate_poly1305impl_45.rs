// Generated macro for impl_45 (impl)
macro_rules! Depcrate_poly1305impl_45 {
() => {
// Module: crate::poly1305
// Provides: {"impl_45"}
// Dependencies: {}
impl Poly1305State { pub fn new (key : & [u8]) -> Poly1305State { assert_eq ! (key . len () , 32) ; let mut ctx : Box < MaybeUninit < ffi :: poly1305_state > > = Box :: new (MaybeUninit :: < ffi :: poly1305_state > :: uninit ()) ; let initialized_ctx : Box < ffi :: poly1305_state > = unsafe { ffi :: CRYPTO_poly1305_init (ctx . as_mut () . as_mut_ptr () , key . as_ptr ()) ; let raw_ctx_ptr = (* Box :: into_raw (ctx)) . as_mut_ptr () ; Box :: from_raw (raw_ctx_ptr) } ; Poly1305State { context : initialized_ctx , } } pub fn update (& mut self , data : & [u8]) { unsafe { ffi :: CRYPTO_poly1305_update (self . context . as_mut () , data . as_ptr () , data . len ()) ; } ; } pub fn finalize (& mut self , output : & mut [u8]) { assert_eq ! (output . len () , 16) ; unsafe { ffi :: CRYPTO_poly1305_finish (self . context . as_mut () , output . as_mut_ptr ()) } ; } }
};
}
