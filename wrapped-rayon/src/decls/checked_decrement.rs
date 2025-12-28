macro_rules! checked_decrement {
    () => {
        fn checked_decrement (u : & AtomicUsize) -> bool { u . fetch_update (Ordering :: Relaxed , Ordering :: Relaxed , | u | u . checked_sub (1)) . is_ok () }
    };
}

checked_decrement!()