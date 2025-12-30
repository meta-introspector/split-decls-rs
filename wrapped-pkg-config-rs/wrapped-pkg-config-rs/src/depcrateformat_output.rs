// Generated macro for format_output (function)
macro_rules! Depcrateformat_output {
() => {
// Module: crate
// Provides: {"format_output"}
// Dependencies: {}
fn format_output (output : & Output , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let stdout = String :: from_utf8_lossy (& output . stdout) ; if ! stdout . is_empty () { write ! (f , "\n--- stdout\n{}" , stdout) ? ; } let stderr = String :: from_utf8_lossy (& output . stderr) ; if ! stderr . is_empty () { write ! (f , "\n--- stderr\n{}" , stderr) ? ; } Ok (()) }
};
}
