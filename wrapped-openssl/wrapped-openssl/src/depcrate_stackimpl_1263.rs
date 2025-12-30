// Generated macro for impl_1263 (impl)
macro_rules! Depcrate_stackimpl_1263 {
() => {
// Module: crate::stack
// Provides: {"impl_1263"}
// Dependencies: {}
impl < T : Stackable > Stack < T > { pub fn new () -> Result < Stack < T > , ErrorStack > { unsafe { ffi :: init () ; let ptr = cvt_p (OPENSSL_sk_new_null ()) ? ; Ok (Stack (ptr as * mut _)) } } }
};
}
