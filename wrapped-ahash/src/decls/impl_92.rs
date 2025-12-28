macro_rules! deps {
    () => {
        CallHasher!();
        RandomState!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        # [cfg (specialize)] impl < T > CallHasher for T where T : Hash + ? Sized , { # [inline] default fn get_hash < H : Hash + ? Sized > (value : & H , random_state : & RandomState) -> u64 { let mut hasher = random_state . build_hasher () ; value . hash (& mut hasher) ; hasher . finish () } }
    };
}

impl_92!()