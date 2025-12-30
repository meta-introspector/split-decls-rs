// Generated macro for ExtractError (enum)
macro_rules! Depcrate_errorExtractError {
() => {
// Module: crate::error
// Provides: {"ExtractError"}
// Dependencies: {}
# [derive (Debug)] pub enum ExtractError < T > { # [doc = " The extracted message was of a different method than expected."] MethodMismatch (T) , # [doc = " Failed to deserialize the message."] JsonError { method : String , error : serde_json :: Error } , }
};
}
