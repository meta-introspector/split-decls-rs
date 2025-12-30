// Generated macro for compute_hash (function)
macro_rules! Depcratecompute_hash {
() => {
// Module: crate
// Provides: {"compute_hash"}
// Dependencies: {}
pub fn compute_hash (content : & str) -> String { use std :: collections :: hash_map :: DefaultHasher ; use std :: hash :: { Hash , Hasher } ; let mut hasher = DefaultHasher :: new () ; content . hash (& mut hasher) ; format ! ("{:08x}" , hasher . finish () & 0xFFFFFFFF) }
};
}
