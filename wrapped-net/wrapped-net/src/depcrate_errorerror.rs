// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " All the errors returned by this crate."] # [derive (Debug , ThisError)] pub enum Error { # [doc = " Error returned by JavaScript."] # [error ("{0}")] JsError (JsError) , # [doc = " Error returned by `serde` during deserialization."] # [cfg (feature = "json")] # [cfg_attr (docsrs , doc (cfg (feature = "json")))] # [error ("{0}")] SerdeError (# [source] # [from] serde_json :: Error ,) , # [doc = " Error returned by this crate"] # [error ("{0}")] GlooError (String) , }
};
}
