// Generated macro for tests (module)
macro_rules! Depcrate_consoletests {
() => {
// Module: crate::console
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , not (target_os = "none")))] mod tests { use super :: * ; # [test] fn test_console () { println ! ("HelloWorld") ; } }
};
}
