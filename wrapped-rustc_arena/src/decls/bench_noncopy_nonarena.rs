macro_rules! deps {
    () => {
        Noncopy!();
    };
}

macro_rules! bench_noncopy_nonarena {
    () => {
        deps!();
        # [bench] fn bench_noncopy_nonarena (b : & mut Bencher) { b . iter (| | { let _ : Box < _ > = Box :: new (Noncopy { string : "hello world" . to_string () , array : vec ! [1 , 2 , 3 , 4 , 5] }) ; }) }
    };
}

bench_noncopy_nonarena!();