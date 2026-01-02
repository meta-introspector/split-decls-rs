
macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub fn fill_bytes (bytes : & mut [u8]) { bytes . copy_from_slice (& wasip2 :: random :: random :: get_random_bytes (u64 :: try_from (bytes . len ()) . unwrap () ,)) ; }
}

macro_rules! hashmap_random_keys_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hashmap_random_keys in module {}", module_path!());
    };
}

mkfn!{
    hashmap_random_keys_introspect!();
    pub fn hashmap_random_keys () -> (u64 , u64) { wasip2 :: random :: insecure_seed :: insecure_seed () }
}