// Generated macro for OsSignals (struct)
macro_rules! Depcrate_signalsOsSignals {
() => {
// Module: crate::signals
// Provides: {"OsSignals"}
// Dependencies: {}
# [doc = " Process signal listener."] pub (crate) struct OsSignals { # [cfg (not (unix))] signals : futures_core :: future :: BoxFuture < 'static , std :: io :: Result < () > > , # [cfg (unix)] signals : Vec < (SignalKind , actix_rt :: signal :: unix :: Signal) > , }
};
}
