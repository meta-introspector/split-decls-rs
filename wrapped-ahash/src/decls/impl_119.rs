macro_rules! deps {
    () => {
        CallHasher!();
        RandomState!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        # [cfg (specialize)] impl CallHasher for [u8] { # [inline] fn get_hash < H : Hash + ? Sized > (value : & H , random_state : & RandomState) -> u64 { random_state . hash_as_str (value) } }
    };
}

impl_119!();