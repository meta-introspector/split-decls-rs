// Generated macro for fmt_array (function)
macro_rules! Depcratefmt_array {
() => {
// Module: crate
// Provides: {"fmt_array"}
// Dependencies: {}
fn fmt_array < T : core :: fmt :: Debug > (array : & [T] , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:?}" , array) }
};
}
