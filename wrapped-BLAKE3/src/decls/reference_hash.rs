macro_rules! deps {
    () => {
        Hash!();
        Hasher!();
    };
}

macro_rules! reference_hash {
    () => {
        deps!();
        fn reference_hash (input : & [u8]) -> crate :: Hash { let mut hasher = reference_impl :: Hasher :: new () ; hasher . update (input) ; let mut bytes = [0 ; 32] ; hasher . finalize (& mut bytes) ; bytes . into () }
    };
}

reference_hash!()