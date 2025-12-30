// Generated macro for check_extend_provenance (function)
macro_rules! Depcrate_testcheck_extend_provenance {
() => {
// Module: crate::test
// Provides: {"check_extend_provenance"}
// Dependencies: {}
# [test] fn check_extend_provenance () { let arena = Arena :: new () ; let a = arena . alloc (0) ; arena . alloc_extend (core :: iter :: once (1)) ; * a = 1 ; }
};
}
