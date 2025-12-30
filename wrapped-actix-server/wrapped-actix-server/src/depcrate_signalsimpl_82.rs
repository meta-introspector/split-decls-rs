// Generated macro for impl_82 (impl)
macro_rules! Depcrate_signalsimpl_82 {
() => {
// Module: crate::signals
// Provides: {"impl_82"}
// Dependencies: {}
impl fmt :: Display for SignalKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { SignalKind :: Cancel => "Cancellation token or channel" , SignalKind :: OsInt => "SIGINT" , SignalKind :: OsTerm => "SIGTERM" , SignalKind :: OsQuit => "SIGQUIT" , }) } }
};
}
