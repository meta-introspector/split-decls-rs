// Generated macro for stream_ref (function)
macro_rules! Depcratestream_ref {
() => {
// Module: crate
// Provides: {"stream_ref"}
// Dependencies: {}
# [doc = "\nStream a value through a stream.\n"] pub fn stream_ref < 'sval , S : Stream < 'sval > > (stream : S , value : & 'sval (impl sval :: Value + ? Sized) ,) -> Result < S :: Ok > { stream . value_ref (value) }
};
}
