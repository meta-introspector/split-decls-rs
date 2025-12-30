// Generated macro for show_error_ (function)
macro_rules! Depcrate_utilshow_error_ {
() => {
// Module: crate::util
// Provides: {"show_error_"}
// Dependencies: {}
pub fn show_error_ (msg : & impl std :: fmt :: Display) -> ! { eprintln ! ("fatal error: {msg}") ; std :: process :: exit (1) }
};
}
