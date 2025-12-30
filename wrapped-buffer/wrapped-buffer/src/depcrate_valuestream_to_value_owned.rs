// Generated macro for stream_to_value_owned (function)
macro_rules! Depcrate_valuestream_to_value_owned {
() => {
// Module: crate::value
// Provides: {"stream_to_value_owned"}
// Dependencies: {}
# [doc = "\nBuffer an owned value.\n"] pub fn stream_to_value_owned (v : impl sval :: Value) -> Result < ValueBuf < 'static > , Error > { ValueBuf :: collect_owned (v) }
};
}
