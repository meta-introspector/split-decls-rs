// Generated macro for test_typed_arena_drop_small_count (function)
macro_rules! Depcrate_teststest_typed_arena_drop_small_count {
() => {
// Module: crate::tests
// Provides: {"test_typed_arena_drop_small_count"}
// Dependencies: {}
# [test] fn test_typed_arena_drop_small_count () { DROP_COUNTER . with (| c | c . set (0)) ; { let arena : TypedArena < SmallDroppable > = TypedArena :: default () ; for _ in 0 .. 100 { arena . alloc (SmallDroppable) ; } } ; assert_eq ! (DROP_COUNTER . with (| c | c . get ()) , 100) ; }
};
}
