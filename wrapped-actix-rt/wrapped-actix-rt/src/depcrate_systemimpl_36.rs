// Generated macro for impl_36 (impl)
macro_rules! Depcrate_systemimpl_36 {
() => {
// Module: crate::system
// Provides: {"impl_36"}
// Dependencies: {}
# [cfg (feature = "io-uring")] impl System { # [doc = " Create a new system."] # [doc = ""] # [doc = " # Panics"] # [doc = " Panics if underlying Tokio runtime can not be created."] # [allow (clippy :: new_ret_no_self)] pub fn new () -> SystemRunner { SystemRunner } # [doc = " Create a new System using the [Tokio Runtime](tokio-runtime) returned from a closure."] # [doc = ""] # [doc = " [tokio-runtime]: tokio::runtime::Runtime"] # [doc (hidden)] pub fn with_tokio_rt < F > (_ : F) -> SystemRunner where F : FnOnce () -> tokio :: runtime :: Runtime , { unimplemented ! ("System::with_tokio_rt is not implemented for io-uring feature yet") } }
};
}
