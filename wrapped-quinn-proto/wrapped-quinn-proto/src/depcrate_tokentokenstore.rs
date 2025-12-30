// Generated macro for TokenStore (trait)
macro_rules! Depcrate_tokenTokenStore {
() => {
// Module: crate::token
// Provides: {"TokenStore"}
// Dependencies: {}
# [doc = " Responsible for storing validation tokens received from servers and retrieving them for use in"] # [doc = " subsequent connections"] pub trait TokenStore : Send + Sync { # [doc = " Potentially store a token for later one-time use"] # [doc = ""] # [doc = " Called when a NEW_TOKEN frame is received from the server."] fn insert (& self , server_name : & str , token : Bytes) ; # [doc = " Try to find and take a token that was stored with the given server name"] # [doc = ""] # [doc = " The same token must never be returned from `take` twice, as doing so can be used to"] # [doc = " de-anonymize a client's traffic."] # [doc = ""] # [doc = " Called when trying to connect to a server. It is always ok for this to return `None`."] fn take (& self , server_name : & str) -> Option < Bytes > ; }
};
}
