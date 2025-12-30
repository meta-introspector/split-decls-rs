// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [allow (variant_size_differences)] # [doc = " An main error type used in [crate]."] # [derive (Debug)] pub enum Error { # [doc = " An Error in IO operation."] IO (io :: Error) , # [doc = " An Error in command line parsing."] CommandParsing , # [doc = " An Error in regex parsing."] RegexParsing , # [doc = " An timeout was reached while waiting in expect call."] ExpectTimeout , # [doc = " Unhandled EOF error."] Eof , # [doc = " It maybe OS specific error or a general erorr."] Other { # [doc = " The reason of the erorr."] message : String , # [doc = " An underlying error message."] err : String , } , }
};
}
