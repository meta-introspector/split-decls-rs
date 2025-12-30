// Generated macro for impl_132 (impl)
macro_rules! Depcrate_busyimpl_132 {
() => {
// Module: crate::busy
// Provides: {"impl_132"}
// Dependencies: {}
impl InnerConnection { # [inline] fn busy_timeout (& mut self , timeout : c_int) -> Result < () > { let r = unsafe { ffi :: sqlite3_busy_timeout (self . db , timeout) } ; self . decode_result (r) } }
};
}
