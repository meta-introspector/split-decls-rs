// Generated macro for stream_to_value (function)
macro_rules! Depcrate_valuestream_to_value {
() => {
// Module: crate::value
// Provides: {"stream_to_value"}
// Dependencies: {}
# [doc = "\nBuffer a value.\n"] pub fn stream_to_value < 'sval > (v : & 'sval (impl sval :: Value + ? Sized) ,) -> Result < ValueBuf < 'sval > , Error > { ValueBuf :: collect (v) }
};
}
