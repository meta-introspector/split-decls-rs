// Generated macro for macro_821 (macro)
macro_rules! Depcrate_stream_stream_peekmacro_821 {
() => {
// Module: crate::stream::stream::peek
// Provides: {"macro_821"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`Peekable::peek`](self::Peekable::peek) method."] # [must_use = "futures do nothing unless polled"] pub struct Peek <'a , St : Stream > { inner : Option < Pin <&'a mut Peekable < St >>>, } }
};
}
