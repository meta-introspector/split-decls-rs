// Generated macro for gnu_hash (function)
macro_rules! Depcrate_elfgnu_hash {
() => {
// Module: crate::elf
// Provides: {"gnu_hash"}
// Dependencies: {}
# [doc = " Calculate the GNU hash for a symbol name."] # [doc = ""] # [doc = " Used for `SHT_GNU_HASH`."] pub fn gnu_hash (name : & [u8]) -> u32 { let mut hash = 5381u32 ; for byte in name { hash = hash . wrapping_mul (33) . wrapping_add (u32 :: from (* byte)) ; } hash }
};
}
