// Generated macro for test_copy (function)
macro_rules! Depcrate_teststest_copy {
() => {
// Module: crate::tests
// Provides: {"test_copy"}
// Dependencies: {}
# [test] fn test_copy () { let arena = TypedArena :: default () ; # [cfg (not (miri))] const N : usize = 100000 ; # [cfg (miri)] const N : usize = 1000 ; for _ in 0 .. N { arena . alloc (Point { x : 1 , y : 2 , z : 3 }) ; } }
};
}
