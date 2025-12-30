// Generated macro for impl_69 (impl)
macro_rules! Depcrate_valueimpl_69 {
() => {
// Module: crate::value
// Provides: {"impl_69"}
// Dependencies: {}
impl ValueBuf < 'static > { # [doc = "\n    Fully buffer a value, including any internal borrowed data.\n\n    This method will fail if the `alloc` feature is not enabled.\n    "] pub fn collect_owned (v : impl sval :: Value) -> Result < Self , Error > { let mut buf = ValueBuf :: new () ; match sval :: stream_computed (& mut buf , v) { Ok (()) => Ok (buf) , Err (_) => Err (buf . into_err () . unwrap_or_else (| | Error :: invalid_value ("the value itself failed to stream"))) , } } }
};
}
