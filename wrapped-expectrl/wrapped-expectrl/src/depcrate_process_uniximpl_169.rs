// Generated macro for impl_169 (impl)
macro_rules! Depcrate_process_uniximpl_169 {
() => {
// Module: crate::process::unix
// Provides: {"impl_169"}
// Dependencies: {}
# [cfg (feature = "async")] impl AsyncPtyStream { fn new (stream : PtyStream) -> Result < Self > { let stream = async_io :: Async :: new (stream) ? ; Ok (Self { stream }) } }
};
}
