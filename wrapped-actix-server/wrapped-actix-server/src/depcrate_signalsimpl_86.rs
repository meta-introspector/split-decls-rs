// Generated macro for impl_86 (impl)
macro_rules! Depcrate_signalsimpl_86 {
() => {
// Module: crate::signals
// Provides: {"impl_86"}
// Dependencies: {}
impl OsSignals { # [doc = " Constructs an OS signal listening future."] pub (crate) fn new () -> Self { trace ! ("setting up OS signal listener") ; # [cfg (not (unix))] { OsSignals { signals : Box :: pin (actix_rt :: signal :: ctrl_c ()) , } } # [cfg (unix)] { use actix_rt :: signal :: unix ; let sig_map = [(unix :: SignalKind :: interrupt () , SignalKind :: OsInt) , (unix :: SignalKind :: terminate () , SignalKind :: OsTerm) , (unix :: SignalKind :: quit () , SignalKind :: OsQuit) ,] ; let signals = sig_map . iter () . filter_map (| (kind , sig) | { unix :: signal (* kind) . map (| tokio_sig | (* sig , tokio_sig)) . map_err (| err | { tracing :: error ! ("can not initialize stream handler for {sig:?} err: {err}" ,) }) . ok () }) . collect :: < Vec < _ > > () ; OsSignals { signals } } } }
};
}
