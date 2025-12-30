// Generated macro for impl_1755 (impl)
macro_rules! Depcrate_tls_boringsslimpl_1755 {
() => {
// Module: crate::tls::boringssl
// Provides: {"impl_1755"}
// Dependencies: {}
impl Context { pub fn set_early_data_enabled (& mut self , _enabled : bool) { unsafe { SSL_CTX_set_early_data_enabled (self . as_mut_ptr () , i32 :: from (_enabled) ,) ; } } }
};
}
