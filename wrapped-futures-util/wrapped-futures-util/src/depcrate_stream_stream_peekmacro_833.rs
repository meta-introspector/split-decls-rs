// Generated macro for macro_833 (macro)
macro_rules! Depcrate_stream_stream_peekmacro_833 {
() => {
// Module: crate::stream::stream::peek
// Provides: {"macro_833"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`Peekable::next_if_eq`](self::Peekable::next_if_eq) method."] # [must_use = "futures do nothing unless polled"] pub struct NextIfEq <'a , St : Stream , T : ? Sized > { # [pin] inner : NextIf <'a , St , NextIfEqFn <'a , T , St :: Item >>, } }
};
}
