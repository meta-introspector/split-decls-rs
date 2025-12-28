macro_rules! deps {
    () => {
        CallHasher!();
        RandomState!();
    };
}

macro_rules! call_hasher_impl_u64 {
    () => {
        deps!();
        macro_rules ! call_hasher_impl_u64 { ($ typ : ty) => { # [cfg (specialize)] impl CallHasher for $ typ { # [inline] fn get_hash < H : Hash + ? Sized > (value : & H , random_state : & RandomState) -> u64 { random_state . hash_as_u64 (value) } } } ; }
    };
}

call_hasher_impl_u64!();