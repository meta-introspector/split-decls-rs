macro_rules! hash {
    () => {
        fn hash < H : Hash , T : Hasher > (b : & H , hash_builder : & dyn Fn () -> T) -> u64 { let mut hasher = hash_builder () ; b . hash (& mut hasher) ; hasher . finish () }
    };
}

hash!();