macro_rules! deps {
    () => {
        RandomState!();
        CallHasher!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        # [cfg (specialize)] impl CallHasher for str { # [inline] fn get_hash < H : Hash + ? Sized > (value : & H , random_state : & RandomState) -> u64 { random_state . hash_as_str (value) } }
    };
}

impl_121!();