macro_rules! deps {
    () => {
        HashValue!();
    };
}

macro_rules! hash_with {
    () => {
        deps!();
        fn hash_with < K , S > (key : & K , build_hasher : & S) -> HashValue where K : ? Sized + Hash , S : BuildHasher , { HashValue (build_hasher . hash_one (key) as u16) }
    };
}

hash_with!();