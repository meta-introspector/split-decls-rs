macro_rules! deps {
    () => {
        AccelerateLookup!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl AccelerateLookup < '_ > { fn with_capacity (cap : usize) -> Self { let ratio_of_entries_to_dirs_in_webkit = 20 ; Self { icase_entries : hashbrown :: HashTable :: with_capacity (cap) , icase_dirs : hashbrown :: HashTable :: with_capacity (cap / ratio_of_entries_to_dirs_in_webkit) , } } fn icase_hash (data : & BStr) -> u64 { use std :: hash :: Hasher ; let mut hasher = fnv :: FnvHasher :: default () ; for b in data . as_bytes () { hasher . write_u8 (b . to_ascii_lowercase ()) ; } hasher . finish () } }
    };
}

impl_114!()