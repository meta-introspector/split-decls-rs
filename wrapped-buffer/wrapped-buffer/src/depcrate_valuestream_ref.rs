// Generated macro for stream_ref (function)
macro_rules! Depcrate_valuestream_ref {
() => {
// Module: crate::value
// Provides: {"stream_ref"}
// Dependencies: {}
fn stream_ref < 'a , 'sval , S : sval :: Stream < 'sval > + ? Sized > (parts : & 'a [ValuePart < 'sval >] , stream : & mut S ,) -> sval :: Result { if parts . len () == 0 { return stream . null () ; } ValueSlice :: new (parts) . stream_ref (stream) }
};
}
