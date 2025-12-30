// Generated macro for evar_equals (function)
macro_rules! Depcrate_colorevar_equals {
() => {
// Module: crate::color
// Provides: {"evar_equals"}
// Dependencies: {}
fn evar_equals (var : Cow < OsStr > , want : & str) -> bool { var == Cow :: Borrowed (OsStr :: new (want)) }
};
}
