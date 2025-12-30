// Generated macro for Error (enum)
macro_rules! Depcrate_serError {
() => {
// Module: crate::ser
// Provides: {"Error"}
// Dependencies: {}
# [derive (Debug , PartialEq)] pub enum Error { SchemaMismatch , # [doc = " Limitations of using serde_json::Value for now"] ShouldSupportButDont , Unsupported , }
};
}
