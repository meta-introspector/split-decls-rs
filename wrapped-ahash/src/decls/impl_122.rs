macro_rules! deps {
    () => {
        CallHasher!();
        RandomState!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        # [cfg (all (specialize))] impl CallHasher for String { # [inline] fn get_hash < H : Hash + ? Sized > (value : & H , random_state : & RandomState) -> u64 { random_state . hash_as_str (value) } }
    };
}

impl_122!()