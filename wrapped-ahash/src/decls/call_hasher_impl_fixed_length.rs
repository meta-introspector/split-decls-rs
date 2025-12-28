macro_rules! deps {
    () => {
        RandomState!();
        CallHasher!();
    };
}

macro_rules! call_hasher_impl_fixed_length {
    () => {
        deps!();
        macro_rules ! call_hasher_impl_fixed_length { ($ typ : ty) => { # [cfg (specialize)] impl CallHasher for $ typ { # [inline] fn get_hash < H : Hash + ? Sized > (value : & H , random_state : & RandomState) -> u64 { random_state . hash_as_fixed_length (value) } } } ; }
    };
}

call_hasher_impl_fixed_length!()