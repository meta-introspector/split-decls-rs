// Generated macro for ScriptError (enum)
macro_rules! Depcrate_errorScriptError {
() => {
// Module: crate::error
// Provides: {"ScriptError"}
// Dependencies: {}
# [cfg (feature = "script_helper")] # [non_exhaustive] # [derive (Debug , Error)] pub enum ScriptError { # [error (transparent)] IoError (# [from] IOError) , # [error (transparent)] ParseError (# [from] ParseError) , }
};
}
