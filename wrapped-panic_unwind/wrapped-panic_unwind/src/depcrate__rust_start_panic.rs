// Generated macro for __rust_start_panic (function)
macro_rules! Depcrate__rust_start_panic {
() => {
// Module: crate
// Provides: {"__rust_start_panic"}
// Dependencies: {}
# [rustc_std_internal_symbol] pub unsafe fn __rust_start_panic (payload : & mut dyn PanicPayload) -> u32 { unsafe { let payload = Box :: from_raw (payload . take_box ()) ; imp :: panic (payload) } }
};
}
