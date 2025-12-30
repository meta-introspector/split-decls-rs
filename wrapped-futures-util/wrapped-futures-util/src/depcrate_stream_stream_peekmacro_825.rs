// Generated macro for macro_825 (macro)
macro_rules! Depcrate_stream_stream_peekmacro_825 {
() => {
// Module: crate::stream::stream::peek
// Provides: {"macro_825"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`Peekable::peek_mut`](self::Peekable::peek_mut) method."] # [must_use = "futures do nothing unless polled"] pub struct PeekMut <'a , St : Stream > { inner : Option < Pin <&'a mut Peekable < St >>>, } }
};
}
