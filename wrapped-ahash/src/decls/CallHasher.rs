macro_rules! deps {
    () => {
        RandomState!();
    };
}

macro_rules! CallHasher {
    () => {
        deps!();
        # [doc = " Provides a way to get an optimized hasher for a given data type."] # [doc = " Rather than using a Hasher generically which can hash any value, this provides a way to get a specialized hash"] # [doc = " for a specific type. So this may be faster for primitive types."] pub (crate) trait CallHasher { fn get_hash < H : Hash + ? Sized > (value : & H , random_state : & RandomState) -> u64 ; }
    };
}

CallHasher!()