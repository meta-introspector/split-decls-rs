macro_rules! deps {
    () => {
        TypedArena!();
        DropCounter!();
    };
}

macro_rules! test_typed_arena_drop_on_clear {
    () => {
        deps!();
        # [test] fn test_typed_arena_drop_on_clear () { let counter = Cell :: new (0) ; let mut arena : TypedArena < DropCounter < '_ > > = TypedArena :: default () ; for i in 0 .. 10 { for _ in 0 .. 100 { arena . alloc (DropCounter { count : & counter }) ; } arena . clear () ; assert_eq ! (counter . get () , i * 100 + 100) ; } }
    };
}

test_typed_arena_drop_on_clear!()