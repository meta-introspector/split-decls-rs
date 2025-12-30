// Generated macro for impl_153 (impl)
macro_rules! Depcrate_workerimpl_153 {
() => {
// Module: crate::worker
// Provides: {"impl_153"}
// Dependencies: {}
impl WorkerHandleServer { pub (crate) fn stop (& self , graceful : bool) -> oneshot :: Receiver < bool > { let (tx , rx) = oneshot :: channel () ; let _ = self . stop_tx . send (Stop { graceful , tx }) ; rx } }
};
}
