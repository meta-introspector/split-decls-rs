macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! hash_node {
    () => {
        deps!();
        # [inline] unsafe fn hash_node < S , K , V > (s : & S , node : NonNull < Node < K , V > >) -> u64 where S : BuildHasher , K : Hash , { hash_key (s , node . as_ref () . key_ref ()) }
    };
}

hash_node!();