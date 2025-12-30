// Generated macro for HistoryError (enum)
macro_rules! Depcrate_errorHistoryError {
() => {
// Module: crate::error
// Provides: {"HistoryError"}
// Dependencies: {}
# [doc = " The Error type for History."] # [derive (Error , Debug)] pub enum HistoryError { # [doc = " Failed to serialize query."] # [cfg (feature = "query")] # [error ("failed to serialize query.")] QuerySer (# [from] serde_urlencoded :: ser :: Error) , # [doc = " Failed to deserialize query."] # [cfg (feature = "query")] # [error ("failed to deserialize query.")] QueryDe (# [from] serde_urlencoded :: de :: Error) , }
};
}
