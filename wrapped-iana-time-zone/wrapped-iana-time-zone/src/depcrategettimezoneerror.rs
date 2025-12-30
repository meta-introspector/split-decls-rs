// Generated macro for GetTimezoneError (enum)
macro_rules! DepcrateGetTimezoneError {
() => {
// Module: crate
// Provides: {"GetTimezoneError"}
// Dependencies: {}
# [doc = " Error types"] # [derive (Debug)] pub enum GetTimezoneError { # [doc = " Failed to parse"] FailedParsingString , # [doc = " Wrapped IO error"] IoError (std :: io :: Error) , # [doc = " Platform-specific error from the operating system"] OsError , }
};
}
