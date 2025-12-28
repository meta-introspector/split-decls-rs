macro_rules! deps {
    () => {
        Point!();
        TypedArena!();
    };
}

macro_rules! bench_typed_arena_clear {
    () => {
        deps!();
        # [bench] fn bench_typed_arena_clear (b : & mut Bencher) { let mut arena = TypedArena :: default () ; b . iter (| | { arena . alloc (Point { x : 1 , y : 2 , z : 3 }) ; arena . clear () ; }) }
    };
}

bench_typed_arena_clear!();