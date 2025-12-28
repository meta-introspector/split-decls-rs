macro_rules! take {
    () => {
        fn take < T > (item : & T , taking : & AtomicBool , predicate : & impl Fn (& T) -> bool) -> bool { if ! taking . load (Ordering :: Relaxed) { return false ; } if predicate (item) { return true ; } taking . store (false , Ordering :: Relaxed) ; false }
    };
}

take!();