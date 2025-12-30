// Generated macro for stream_display (function)
macro_rules! Depcrate_to_valuestream_display {
() => {
// Module: crate::to_value
// Provides: {"stream_display"}
// Dependencies: {}
# [doc = "\nStream a [`fmt::Display`] into an [`sval::Stream`].\n"] pub fn stream_display < 'sval > (stream : & mut (impl sval :: Stream < 'sval > + ? Sized) , value : impl fmt :: Display ,) -> sval :: Result { stream . value_computed (& DisplayToValue :: new (value)) }
};
}
