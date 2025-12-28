macro_rules! deps {
    () => {
        Noncopy!();
        TypedArena!();
    };
}

macro_rules! test_noncopy {
    () => {
        deps!();
        # [test] fn test_noncopy () { let arena = TypedArena :: default () ; # [cfg (not (miri))] const N : usize = 100000 ; # [cfg (miri)] const N : usize = 1000 ; for _ in 0 .. N { arena . alloc (Noncopy { string : "hello world" . to_string () , array : vec ! [1 , 2 , 3 , 4 , 5] }) ; } }
    };
}

test_noncopy!();