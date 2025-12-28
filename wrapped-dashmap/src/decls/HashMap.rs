macro_rules! HashMap {
    () => {
        pub (crate) type HashMap < K , V > = hash_table :: HashTable < (K , V) > ;
    };
}

HashMap!();