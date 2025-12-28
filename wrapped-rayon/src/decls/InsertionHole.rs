macro_rules! InsertionHole {
    () => {
        struct InsertionHole < T > { src : * const T , dest : * mut T , }
    };
}

InsertionHole!();