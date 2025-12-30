// Generated macro for RawEntryBuilder (struct)
macro_rules! Depcrate_raw_entryRawEntryBuilder {
() => {
// Module: crate::raw_entry
// Provides: {"RawEntryBuilder"}
// Dependencies: {}
# [doc = " A builder for computing where in a [`HashMap`] a key-value pair would be stored."] # [doc = ""] # [doc = " See the [`HashMap::raw_entry`] docs for usage examples."] # [doc = ""] # [doc = " [`HashMap::raw_entry`]: struct.HashMap.html#method.raw_entry"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::hash_map::{HashMap, RawEntryBuilder};"] # [doc = " use core::hash::{BuildHasher, Hash};"] # [doc = ""] # [doc = " let mut map = HashMap::new();"] # [doc = " map.extend([(1, 10), (2, 20), (3, 30)]);"] # [doc = ""] # [doc = " fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {"] # [doc = "     use core::hash::Hasher;"] # [doc = "     let mut state = hash_builder.build_hasher();"] # [doc = "     key.hash(&mut state);"] # [doc = "     state.finish()"] # [doc = " }"] # [doc = ""] # [doc = " for k in 0..6 {"] # [doc = "     let hash = compute_hash(map.hasher(), &k);"] # [doc = "     let v = map.get(&k).cloned();"] # [doc = "     let kv = v.as_ref().map(|v| (&k, v));"] # [doc = ""] # [doc = "     println!(\"Key: {} and value: {:?}\", k, v);"] # [doc = "     let builder: RawEntryBuilder<_, _, _> = map.raw_entry();"] # [doc = "     assert_eq!(builder.from_key(&k), kv);"] # [doc = "     assert_eq!(map.raw_entry().from_hash(hash, |q| *q == k), kv);"] # [doc = "     assert_eq!(map.raw_entry().from_key_hashed_nocheck(hash, &k), kv);"] # [doc = " }"] # [doc = " ```"] pub struct RawEntryBuilder < 'a , K , V , S , A : Allocator = Global > { map : & 'a HashMap < K , V , S , A > , }
};
}
