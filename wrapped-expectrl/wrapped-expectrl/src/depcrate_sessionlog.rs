// Generated macro for log (function)
macro_rules! Depcrate_sessionlog {
() => {
// Module: crate::session
// Provides: {"log"}
// Dependencies: {}
# [doc = " Set a logger which will write each Read/Write operation into the writter."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use expectrl::{spawn, session::log};"] # [doc = ""] # [doc = " let p = spawn(\"cat\").unwrap();"] # [doc = " let p = log(p, std::io::stdout());"] # [doc = " ```"] # [cfg (feature = "async")] pub fn log < W , P , S > (session : Session < P , S > , dst : W) -> Result < Session < P , LogStream < S , W > > , Error > where W : Write , { session . swap_stream (| s | LogStream :: new (s , dst)) }
};
}
