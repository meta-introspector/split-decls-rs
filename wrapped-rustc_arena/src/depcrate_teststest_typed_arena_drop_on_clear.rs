// Generated macro for test_typed_arena_drop_on_clear (function)
macro_rules! Depcrate_teststest_typed_arena_drop_on_clear {
() => {
// Module: crate::tests
// Provides: {"test_typed_arena_drop_on_clear"}
// Dependencies: {}
# [test] fn test_typed_arena_drop_on_clear () { let counter = Cell :: new (0) ; let mut arena : TypedArena < DropCounter < '_ > > = TypedArena :: default () ; for i in 0 .. 10 { for _ in 0 .. 100 { arena . alloc (DropCounter { count : & counter }) ; } arena . clear () ; assert_eq ! (counter . get () , i * 100 + 100) ; } }
};
}
