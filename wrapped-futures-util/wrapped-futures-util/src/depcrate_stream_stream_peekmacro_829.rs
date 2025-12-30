// Generated macro for macro_829 (macro)
macro_rules! Depcrate_stream_stream_peekmacro_829 {
() => {
// Module: crate::stream::stream::peek
// Provides: {"macro_829"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`Peekable::next_if`](self::Peekable::next_if) method."] # [must_use = "futures do nothing unless polled"] pub struct NextIf <'a , St : Stream , F > { inner : Option < (Pin <&'a mut Peekable < St >>, F) >, } }
};
}
