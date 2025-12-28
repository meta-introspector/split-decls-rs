macro_rules! force_fallback {
    () => {
        pub (crate) fn force_fallback () { WORKS . store (1 , Ordering :: Relaxed) ; }
    };
}

force_fallback!();