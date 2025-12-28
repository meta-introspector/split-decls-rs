macro_rules! deps {
    () => {
        CallHasher!();
        RandomState!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        # [cfg (not (specialize))] impl < T > CallHasher for T where T : Hash + ? Sized , { # [inline] fn get_hash < H : Hash + ? Sized > (value : & H , random_state : & RandomState) -> u64 { let mut hasher = random_state . build_hasher () ; value . hash (& mut hasher) ; hasher . finish () } }
    };
}

impl_91!()