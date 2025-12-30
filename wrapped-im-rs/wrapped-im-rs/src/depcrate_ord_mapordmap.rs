// Generated macro for OrdMap (struct)
macro_rules! Depcrate_ord_mapOrdMap {
() => {
// Module: crate::ord::map
// Provides: {"OrdMap"}
// Dependencies: {}
# [doc = " An ordered map."] # [doc = ""] # [doc = " An immutable ordered map implemented as a B-tree."] # [doc = ""] # [doc = " Most operations on this type of map are O(log n). A"] # [doc = " [`HashMap`][hashmap::HashMap] is usually a better choice for"] # [doc = " performance, but the `OrdMap` has the advantage of only requiring"] # [doc = " an [`Ord`][std::cmp::Ord] constraint on the key, and of being"] # [doc = " ordered, so that keys always come out from lowest to highest,"] # [doc = " where a [`HashMap`][hashmap::HashMap] has no guaranteed ordering."] # [doc = ""] # [doc = " [hashmap::HashMap]: ../hashmap/struct.HashMap.html"] # [doc = " [std::cmp::Ord]: https://doc.rust-lang.org/std/cmp/trait.Ord.html"] pub struct OrdMap < K , V > { size : usize , pool : OrdMapPool < K , V > , root : PoolRef < Node < (K , V) > > , }
};
}
