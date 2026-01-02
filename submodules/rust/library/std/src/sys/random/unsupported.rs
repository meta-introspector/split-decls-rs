mkuse!{use crate :: ptr ;}

macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub fn fill_bytes (_ : & mut [u8]) { panic ! ("this target does not support random data generation") ; }
}

macro_rules! hashmap_random_keys_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hashmap_random_keys in module {}", module_path!());
    };
}

mkfn!{
    hashmap_random_keys_introspect!();
    pub fn hashmap_random_keys () -> (u64 , u64) { let stack = 0u8 ; let heap = Box :: new (0u8) ; let k1 = ptr :: from_ref (& stack) . addr () as u64 ; let k2 = ptr :: from_ref (& * heap) . addr () as u64 ; (k1 , k2) }
}