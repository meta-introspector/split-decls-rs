// Generated macro for fixture (function)
macro_rules! Depcrate_testfixture {
() => {
// Module: crate::test
// Provides: {"fixture"}
// Dependencies: {}
pub (crate) fn fixture (name : impl AsRef < str > , args : & [& str]) -> Fixture { let name = name . as_ref () . to_owned () ; Fixture :: new (pat (& name) , path (& name) , Positional (to_exprs ! (args))) }
};
}
