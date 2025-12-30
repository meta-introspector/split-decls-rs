// Generated macro for Backend (trait)
macro_rules! Depcrate_ffiBackend {
() => {
// Module: crate::ffi
// Provides: {"Backend"}
// Dependencies: {}
# [doc = " Traits specifying the interface of the backends."] # [doc = ""] # [doc = " Sync + Send are added as a condition to ensure they are available"] # [doc = " for the frontend."] pub trait Backend : Sync + Send { fn total_in (& self) -> u64 ; fn total_out (& self) -> u64 ; }
};
}
