// Generated macro for tests (module)
macro_rules! Depcrate___macros_classtests {
() => {
// Module: crate::__macros::class
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] # [should_panic = "class NonExistentClass could not be found"] # [cfg (not (feature = "unstable-static-class"))] fn test_not_found () { let _ = crate :: class ! (NonExistentClass) ; } }
};
}
