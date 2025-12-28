macro_rules! deps {
    () => {
        Point!();
    };
}

macro_rules! bench_copy_nonarena {
    () => {
        deps!();
        # [bench] fn bench_copy_nonarena (b : & mut Bencher) { b . iter (| | { let _ : Box < _ > = Box :: new (Point { x : 1 , y : 2 , z : 3 }) ; }) }
    };
}

bench_copy_nonarena!()