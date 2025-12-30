// Generated macro for calculate_hash (function)
macro_rules! Depcrate_match_groupcalculate_hash {
() => {
// Module: crate::match_group
// Provides: {"calculate_hash"}
// Dependencies: {}
fn calculate_hash < T : std :: hash :: Hash > (t : & T) -> u64 { use std :: hash :: Hasher ; let mut s = std :: collections :: hash_map :: DefaultHasher :: new () ; t . hash (& mut s) ; s . finish () }
};
}
