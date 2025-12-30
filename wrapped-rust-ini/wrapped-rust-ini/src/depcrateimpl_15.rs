// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl LineSeparator { # [doc = " String representation"] pub fn as_str (self) -> & 'static str { match self { LineSeparator :: SystemDefault => DEFAULT_LINE_SEPARATOR , LineSeparator :: CR => "\n" , LineSeparator :: CRLF => "\r\n" , } } }
};
}
