// Generated macro for impl_20 (impl)
macro_rules! Depcrate_cmacimpl_20 {
() => {
// Module: crate::cmac
// Provides: {"impl_20"}
// Dependencies: {}
impl Cmac { pub fn new (key : & [u8] , cipher : & openssl :: cipher :: CipherRef) -> OpenSSLResult < Cmac > { unsafe { let ctx = Cmac :: from_ptr (cvt_p (ffi :: CMAC_CTX_new ()) ?) ; cvt (ffi :: CMAC_Init (ctx . as_ptr () , key . as_ptr () . cast () , key . len () , cipher . as_ptr () , ptr :: null_mut () ,)) ? ; Ok (ctx) } } }
};
}
