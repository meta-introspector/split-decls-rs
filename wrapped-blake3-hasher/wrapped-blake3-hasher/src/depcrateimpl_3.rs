// Generated macro for impl_3 (impl)
macro_rules! Depcrateimpl_3 {
() => {
// Module: crate
// Provides: {"impl_3"}
// Dependencies: {}
# [cfg (all (feature = "blake3" , not (any (target_os = "solana" , target_arch = "bpf"))))] impl Hasher { pub fn hash (& mut self , val : & [u8]) { self . hasher . update (val) ; } pub fn hashv (& mut self , vals : & [& [u8]]) { for val in vals { self . hash (val) ; } } pub fn result (self) -> Hash { Hash :: new_from_array (* self . hasher . finalize () . as_bytes ()) } }
};
}
