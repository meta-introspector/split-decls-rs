// Generated macro for async_mode (function)
macro_rules! Depcrateasync_mode {
() => {
// Module: crate
// Provides: {"async_mode"}
// Dependencies: {}
fn async_mode (arg : & str) -> Result < AsyncTraitMode > { match arg { "" | "Send" => Ok (AsyncTraitMode :: Send) , "?Send" => Ok (AsyncTraitMode :: NotSend) , "AFIT" => Ok (AsyncTraitMode :: Off) , _ => Err (syn :: Error :: new (Span :: call_site () , "Only accepts `Send`, `?Send` or `AFIT` (native async function in trait)" ,)) , } }
};
}
