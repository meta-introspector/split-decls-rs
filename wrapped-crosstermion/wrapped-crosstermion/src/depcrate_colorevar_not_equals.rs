// Generated macro for evar_not_equals (function)
macro_rules! Depcrate_colorevar_not_equals {
() => {
// Module: crate::color
// Provides: {"evar_not_equals"}
// Dependencies: {}
fn evar_not_equals (var : Cow < OsStr > , want : & str) -> bool { var != Cow :: Borrowed (OsStr :: new (want)) }
};
}
