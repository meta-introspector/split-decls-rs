// Generated macro for Error (enum)
macro_rules! Depcrate_protocolError {
() => {
// Module: crate::protocol
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned top-level credential functions."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] UrlParse (# [from] gix_url :: parse :: Error) , # [error ("Either 'url' field or both 'protocol' and 'host' fields must be provided")] UrlMissing , # [error (transparent)] ContextDecode (# [from] context :: decode :: Error) , # [error (transparent)] InvokeHelper (# [from] helper :: Error) , # [error ("Could not obtain identity for context: {}" , { let mut buf = Vec ::< u8 >:: new () ; context . write_to (& mut buf) . ok () ; String :: from_utf8_lossy (& buf) . into_owned () })] IdentityMissing { context : Context } , # [error ("The handler asked to stop trying to obtain credentials")] Quit , # [error ("Couldn't obtain {prompt}")] Prompt { prompt : String , source : gix_prompt :: Error } , }
};
}
