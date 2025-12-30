// Generated macro for Stop (struct)
macro_rules! Depcrate_workerStop {
() => {
// Module: crate::worker
// Provides: {"Stop"}
// Dependencies: {}
# [doc = " Stop worker message. Returns `true` on successful graceful shutdown"] # [doc = " and `false` if some connections still alive when shutdown execute."] pub (crate) struct Stop { graceful : bool , tx : oneshot :: Sender < bool > , }
};
}
