macro_rules! Pair {
    () => {
        enum Pair < T > { Both (T , T) , One (T) , None , }
    };
}

Pair!();