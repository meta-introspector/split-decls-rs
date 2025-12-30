// Generated macro for IncomingToken (struct)
macro_rules! Depcrate_tokenIncomingToken {
() => {
// Module: crate::token
// Provides: {"IncomingToken"}
// Dependencies: {}
# [doc = " State in an `Incoming` determined by a token or lack thereof"] # [derive (Debug)] pub (crate) struct IncomingToken { pub (crate) retry_src_cid : Option < ConnectionId > , pub (crate) orig_dst_cid : ConnectionId , pub (crate) validated : bool , }
};
}
