// Generated macro for make_hash (function)
macro_rules! Depcrate_mapmake_hash {
() => {
// Module: crate::map
// Provides: {"make_hash"}
// Dependencies: {}
# [cfg (feature = "nightly")] # [cfg_attr (feature = "inline-more" , inline)] pub (crate) fn make_hash < Q , S > (hash_builder : & S , val : & Q) -> u64 where Q : Hash + ? Sized , S : BuildHasher , { hash_builder . hash_one (val) }
};
}
