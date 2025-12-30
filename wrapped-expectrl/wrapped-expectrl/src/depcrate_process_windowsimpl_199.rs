// Generated macro for impl_199 (impl)
macro_rules! Depcrate_process_windowsimpl_199 {
() => {
// Module: crate::process::windows
// Provides: {"impl_199"}
// Dependencies: {}
# [cfg (feature = "async")] impl AsyncProcessStream { fn new (stream : ProcessStream) -> Result < Self > { let input = blocking :: Unblock :: new (stream . input) ; let output = blocking :: Unblock :: new (stream . output) ; Ok (Self { input , output }) } }
};
}
