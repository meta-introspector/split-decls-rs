// Generated macro for wasmtime_page_size (function)
macro_rules! Depcrate_capiwasmtime_page_size {
() => {
// Module: crate::capi
// Provides: {"wasmtime_page_size"}
// Dependencies: {}
# [doc = " Returns the page size, in bytes, of the current system."] # [unsafe (no_mangle)] pub extern "C" fn wasmtime_page_size () -> usize { unsafe { hermit_abi :: getpagesize () . try_into () . unwrap () } }
};
}
