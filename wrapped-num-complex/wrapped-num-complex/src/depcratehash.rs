// Generated macro for hash (function)
macro_rules! Depcratehash {
() => {
// Module: crate
// Provides: {"hash"}
// Dependencies: {}
# [cfg (test)] fn hash < T : hash :: Hash > (x : & T) -> u64 { use std :: collections :: hash_map :: RandomState ; use std :: hash :: { BuildHasher , Hasher } ; let mut hasher = < RandomState as BuildHasher > :: Hasher :: new () ; x . hash (& mut hasher) ; hasher . finish () }
};
}
