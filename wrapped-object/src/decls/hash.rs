macro_rules! hash {
    () => {
        # [doc = " Calculate the SysV hash for a symbol name."] # [doc = ""] # [doc = " Used for `SHT_HASH`."] pub fn hash (name : & [u8]) -> u32 { let mut hash = 0u32 ; for byte in name { hash = hash . wrapping_mul (16) . wrapping_add (u32 :: from (* byte)) ; hash ^= (hash >> 24) & 0xf0 ; } hash & 0xfff_ffff }
    };
}

hash!()