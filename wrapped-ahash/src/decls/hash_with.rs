macro_rules! hash_with {
    () => {
        fn hash_with < H : Hash , T : Hasher > (b : & H , mut hasher : T) -> u64 { b . hash (& mut hasher) ; hasher . finish () }
    };
}

hash_with!();