// Generated macro for Error (enum)
macro_rules! Depcrate_contentError {
() => {
// Module: crate::content
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An internal error type for content related errors."] # [derive (Debug)] pub enum Error { FailedParsingYaml (std :: path :: PathBuf) , UnexpectedDataType , MissingField , FileIo (std :: io :: Error , std :: path :: PathBuf) , }
};
}
