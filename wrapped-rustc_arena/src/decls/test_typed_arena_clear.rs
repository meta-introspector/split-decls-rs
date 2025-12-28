macro_rules! deps {
    () => {
        TypedArena!();
        Point!();
    };
}

macro_rules! test_typed_arena_clear {
    () => {
        deps!();
        # [test] fn test_typed_arena_clear () { let mut arena = TypedArena :: default () ; for _ in 0 .. 10 { arena . clear () ; # [cfg (not (miri))] const N : usize = 10000 ; # [cfg (miri)] const N : usize = 100 ; for _ in 0 .. N { arena . alloc (Point { x : 1 , y : 2 , z : 3 }) ; } } }
    };
}

test_typed_arena_clear!();