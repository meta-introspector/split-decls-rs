// Generated macro for HashSet (struct)
macro_rules! Depcrate_hash_setHashSet {
() => {
// Module: crate::hash::set
// Provides: {"HashSet"}
// Dependencies: {}
# [doc = " An unordered set."] # [doc = ""] # [doc = " An immutable hash set using [hash array mapped tries] [1]."] # [doc = ""] # [doc = " Most operations on this set are O(log<sub>x</sub> n) for a"] # [doc = " suitably high *x* that it should be nearly O(1) for most sets."] # [doc = " Because of this, it's a great choice for a generic set as long as"] # [doc = " you don't mind that values will need to implement"] # [doc = " [`Hash`][std::hash::Hash] and [`Eq`][std::cmp::Eq]."] # [doc = ""] # [doc = " Values will have a predictable order based on the hasher"] # [doc = " being used. Unless otherwise specified, this will be the standard"] # [doc = " [`RandomState`][std::collections::hash_map::RandomState] hasher."] # [doc = ""] # [doc = " [1]: https://en.wikipedia.org/wiki/Hash_array_mapped_trie"] # [doc = " [std::cmp::Eq]: https://doc.rust-lang.org/std/cmp/trait.Eq.html"] # [doc = " [std::hash::Hash]: https://doc.rust-lang.org/std/hash/trait.Hash.html"] # [doc = " [std::collections::hash_map::RandomState]: https://doc.rust-lang.org/std/collections/hash_map/struct.RandomState.html"] pub struct HashSet < A , S = RandomState > { hasher : Ref < S > , pool : HashSetPool < A > , root : PoolRef < Node < Value < A > > > , size : usize , }
};
}
