macro_rules! scope_fifo_spawn_broadcast {
    () => {
        # [test] fn scope_fifo_spawn_broadcast () { let sum = AtomicUsize :: new (0) ; let n = scope_fifo (| s | { s . spawn_broadcast (| _ , ctx | { sum . fetch_add (ctx . index () , Ordering :: Relaxed) ; }) ; crate :: current_num_threads () }) ; assert_eq ! (sum . into_inner () , n * (n - 1) / 2) ; }
    };
}

scope_fifo_spawn_broadcast!();