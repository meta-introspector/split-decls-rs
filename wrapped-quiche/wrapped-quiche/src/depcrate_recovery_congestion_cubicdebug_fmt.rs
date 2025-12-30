// Generated macro for debug_fmt (function)
macro_rules! Depcrate_recovery_congestion_cubicdebug_fmt {
() => {
// Module: crate::recovery::congestion::cubic
// Provides: {"debug_fmt"}
// Dependencies: {}
fn debug_fmt (r : & Congestion , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (f , "cubic={{ k={} w_max={} }} " , r . cubic_state . k , r . cubic_state . w_max) }
};
}
