// Generated macro for stream_debug (function)
macro_rules! Depcrate_to_valuestream_debug {
() => {
// Module: crate::to_value
// Provides: {"stream_debug"}
// Dependencies: {}
# [doc = "\nStream a [`fmt::Debug`] into an [`sval::Stream`].\n"] pub fn stream_debug < 'sval > (stream : & mut (impl sval :: Stream < 'sval > + ? Sized) , value : impl fmt :: Debug ,) -> sval :: Result { stream . value_computed (& DebugToValue :: new (value)) }
};
}
