// Generated macro for ResetTokenTable (struct)
macro_rules! Depcrate_endpointResetTokenTable {
() => {
// Module: crate::endpoint
// Provides: {"ResetTokenTable"}
// Dependencies: {}
# [doc = " Reset Tokens which are associated with peer socket addresses"] # [doc = ""] # [doc = " The standard `HashMap` is used since both `SocketAddr` and `ResetToken` are"] # [doc = " peer generated and might be usable for hash collision attacks."] # [derive (Default , Debug)] struct ResetTokenTable (HashMap < SocketAddr , HashMap < ResetToken , ConnectionHandle > >) ;
};
}
