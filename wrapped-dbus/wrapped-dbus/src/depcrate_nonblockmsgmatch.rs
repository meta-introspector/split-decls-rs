// Generated macro for MsgMatch (struct)
macro_rules! Depcrate_nonblockMsgMatch {
() => {
// Module: crate::nonblock
// Provides: {"MsgMatch"}
// Dependencies: {}
# [doc = " A struct used to handle incoming matches"] # [doc = ""] # [doc = " Note: Due to the lack of async destructors, please call Connection.remove_match()"] # [doc = " in order to properly stop matching (instead of just dropping this struct)."] pub struct MsgMatch (Arc < MatchInner >) ;
};
}
