macro_rules! transition {
    () => {
        fn transition (current : usize , new : usize) -> bool { static STATE : AtomicUsize = AtomicUsize :: new (OFF) ; STATE . compare_exchange (current , new , Ordering :: SeqCst , Ordering :: SeqCst) . is_ok () }
    };
}

transition!();