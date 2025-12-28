macro_rules! deps {
    () => {
        Noncopy!();
        TypedArena!();
    };
}

macro_rules! bench_noncopy {
    () => {
        deps!();
        # [bench] fn bench_noncopy (b : & mut Bencher) { let arena = TypedArena :: default () ; b . iter (| | { arena . alloc (Noncopy { string : "hello world" . to_string () , array : vec ! [1 , 2 , 3 , 4 , 5] }) }) }
    };
}

bench_noncopy!();