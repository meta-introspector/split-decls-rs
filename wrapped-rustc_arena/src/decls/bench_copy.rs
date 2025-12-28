macro_rules! deps {
    () => {
        Point!();
        TypedArena!();
    };
}

macro_rules! bench_copy {
    () => {
        deps!();
        # [bench] fn bench_copy (b : & mut Bencher) { let arena = TypedArena :: default () ; b . iter (| | arena . alloc (Point { x : 1 , y : 2 , z : 3 })) }
    };
}

bench_copy!()