// Generated macro for ReadyState (struct)
macro_rules! Depcrate_async_readyReadyState {
() => {
// Module: crate::async_ready
// Provides: {"ReadyState"}
// Dependencies: {}
struct ReadyState < T : Async > { set_completed : AtomicBool , result : Result < T :: Output > , }
};
}
