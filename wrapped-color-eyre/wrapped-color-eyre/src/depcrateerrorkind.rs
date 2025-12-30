// Generated macro for ErrorKind (enum)
macro_rules! DepcrateErrorKind {
() => {
// Module: crate
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " The kind of type erased error being reported"] # [cfg (feature = "issue-url")] # [cfg_attr (docsrs , doc (cfg (feature = "issue-url")))] pub enum ErrorKind < 'a > { # [doc = " A non recoverable error aka `panic!`"] NonRecoverable (& 'a dyn std :: any :: Any) , # [doc = " A recoverable error aka `impl std::error::Error`"] Recoverable (& 'a (dyn std :: error :: Error + 'static)) , }
};
}
