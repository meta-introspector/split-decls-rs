// Generated macro for stream_ref (function)
macro_rules! Depcratestream_ref {
() => {
// Module: crate
// Provides: {"stream_ref"}
// Dependencies: {}
# [doc = "\nStream a value through a stream.\n"] pub fn stream_ref < 'sval > (stream : & mut (impl Stream < 'sval > + ? Sized) , value : impl ValueRef < 'sval > ,) -> Result { value . stream_ref (stream) }
};
}
