// Generated macro for Drain (struct)
macro_rules! Depcrate_mapDrain {
() => {
// Module: crate::map
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " A draining iterator over the entries of a `HashMap` in arbitrary"] # [doc = " order. The iterator element type is `(K, V)`."] # [doc = ""] # [doc = " This `struct` is created by the [`drain`] method on [`HashMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`drain`]: struct.HashMap.html#method.drain"] # [doc = " [`HashMap`]: struct.HashMap.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = ""] # [doc = " let mut map: HashMap<_, _> = [(1, \"a\"), (2, \"b\"), (3, \"c\")].into();"] # [doc = ""] # [doc = " let mut drain_iter = map.drain();"] # [doc = " let mut vec = vec![drain_iter.next(), drain_iter.next(), drain_iter.next()];"] # [doc = ""] # [doc = " // The `Drain` iterator produces items in arbitrary order, so the"] # [doc = " // items must be sorted to test them against a sorted array."] # [doc = " vec.sort_unstable();"] # [doc = " assert_eq!(vec, [Some((1, \"a\")), Some((2, \"b\")), Some((3, \"c\"))]);"] # [doc = ""] # [doc = " // It is fused iterator"] # [doc = " assert_eq!(drain_iter.next(), None);"] # [doc = " assert_eq!(drain_iter.next(), None);"] # [doc = " ```"] pub struct Drain < 'a , K , V , A : Allocator = Global > { inner : RawDrain < 'a , (K , V) , A > , }
};
}
