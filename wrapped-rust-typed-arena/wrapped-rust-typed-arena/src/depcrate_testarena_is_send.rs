// Generated macro for arena_is_send (function)
macro_rules! Depcrate_testarena_is_send {
() => {
// Module: crate::test
// Provides: {"arena_is_send"}
// Dependencies: {}
# [test] fn arena_is_send () { fn assert_is_send < T : Send > (_ : T) { } assert_is_send (42_u32) ; let arena : Arena < u32 > = Arena :: new () ; assert_is_send (arena) ; }
};
}
