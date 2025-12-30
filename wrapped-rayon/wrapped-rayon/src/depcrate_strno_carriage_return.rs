// Generated macro for no_carriage_return (function)
macro_rules! Depcrate_strno_carriage_return {
() => {
// Module: crate::str
// Provides: {"no_carriage_return"}
// Dependencies: {}
# [inline] fn no_carriage_return (line : & str) -> & str { line . strip_suffix ('\r') . unwrap_or (line) }
};
}
