// Generated macro for to_io_error (function)
macro_rules! Depcrate_errorto_io_error {
() => {
// Module: crate::error
// Provides: {"to_io_error"}
// Dependencies: {}
pub (crate) fn to_io_error < E : Display > (message : & 'static str) -> impl FnOnce (E) -> io :: Error { move | e : E | io :: Error :: other (format ! ("{}; {}" , message , e)) }
};
}
