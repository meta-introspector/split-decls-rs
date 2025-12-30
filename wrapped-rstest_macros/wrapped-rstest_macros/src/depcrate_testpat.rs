// Generated macro for pat (function)
macro_rules! Depcrate_testpat {
() => {
// Module: crate::test
// Provides: {"pat"}
// Dependencies: {}
pub (crate) fn pat (s : impl AsRef < str >) -> syn :: Pat { syn :: parse :: Parser :: parse_str (Pat :: parse_single , s . as_ref ()) . unwrap () }
};
}
