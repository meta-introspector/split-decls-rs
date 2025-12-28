macro_rules! skip {
    () => {
        fn skip < T > (item : & T , skipping : & AtomicBool , predicate : & impl Fn (& T) -> bool) -> bool { if ! skipping . load (Ordering :: Relaxed) { return false ; } if predicate (item) { return true ; } skipping . store (false , Ordering :: Relaxed) ; false }
    };
}

skip!()