macro_rules! deps {
    () => {
        TypedArena!();
        SmallDroppable!();
    };
}

macro_rules! test_typed_arena_drop_small_count {
    () => {
        deps!();
        # [test] fn test_typed_arena_drop_small_count () { DROP_COUNTER . with (| c | c . set (0)) ; { let arena : TypedArena < SmallDroppable > = TypedArena :: default () ; for _ in 0 .. 100 { arena . alloc (SmallDroppable) ; } } ; assert_eq ! (DROP_COUNTER . with (| c | c . get ()) , 100) ; }
    };
}

test_typed_arena_drop_small_count!();