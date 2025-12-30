// Generated macro for impl_41 (impl)
macro_rules! Depcrate_systemimpl_41 {
() => {
// Module: crate::system
// Provides: {"impl_41"}
// Dependencies: {}
# [cfg (feature = "io-uring")] impl SystemRunner { # [doc = " Starts event loop and will return once [System] is [stopped](System::stop)."] pub fn run (self) -> io :: Result < () > { unimplemented ! ("SystemRunner::run is not implemented for io-uring feature yet") ; } # [doc = " Runs the event loop until [stopped](System::stop_with_code), returning the exit code."] pub fn run_with_code (self) -> io :: Result < i32 > { unimplemented ! ("SystemRunner::run_with_code is not implemented for io-uring feature yet") ; } # [doc = " Runs the provided future, blocking the current thread until the future completes."] # [inline] pub fn block_on < F : Future > (& self , fut : F) -> F :: Output { tokio_uring :: start (async move { let (stop_tx , stop_rx) = oneshot :: channel () ; let (sys_tx , sys_rx) = mpsc :: unbounded_channel () ; let sys_arbiter = Arbiter :: in_new_system () ; let system = System :: construct (sys_tx , sys_arbiter . clone ()) ; system . tx () . send (SystemCommand :: RegisterArbiter (usize :: MAX , sys_arbiter)) . unwrap () ; let sys_ctrl = SystemController :: new (sys_rx , stop_tx) ; tokio_uring :: spawn (sys_ctrl) ; let res = fut . await ; drop (stop_rx) ; res }) } }
};
}
