macro_rules! NEXT_MAP_INDEX {
    () => {
        static NEXT_MAP_INDEX : AtomicUsize = AtomicUsize :: new (0) ;
    };
}

NEXT_MAP_INDEX!()