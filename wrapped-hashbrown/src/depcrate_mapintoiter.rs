// Generated macro for IntoIter (struct)
macro_rules! Depcrate_mapIntoIter {
() => {
// Module: crate::map
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An owning iterator over the entries of a `HashMap` in arbitrary order."] # [doc = " The iterator element type is `(K, V)`."] # [doc = ""] # [doc = " This `struct` is created by the [`into_iter`] method on [`HashMap`]"] # [doc = " (provided by the [`IntoIterator`] trait). See its documentation for more."] # [doc = " The map cannot be used after calling that method."] # [doc = ""] # [doc = " [`into_iter`]: struct.HashMap.html#method.into_iter"] # [doc = " [`HashMap`]: struct.HashMap.html"] # [doc = " [`IntoIterator`]: https://doc.rust-lang.org/core/iter/trait.IntoIterator.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = ""] # [doc = " let map: HashMap<_, _> = [(1, \"a\"), (2, \"b\"), (3, \"c\")].into();"] # [doc = ""] # [doc = " let mut iter = map.into_iter();"] # [doc = " let mut vec = vec![iter.next(), iter.next(), iter.next()];"] # [doc = ""] # [doc = " // The `IntoIter` iterator produces items in arbitrary order, so the"] # [doc = " // items must be sorted to test them against a sorted array."] # [doc = " vec.sort_unstable();"] # [doc = " assert_eq!(vec, [Some((1, \"a\")), Some((2, \"b\")), Some((3, \"c\"))]);"] # [doc = ""] # [doc = " // It is fused iterator"] # [doc = " assert_eq!(iter.next(), None);"] # [doc = " assert_eq!(iter.next(), None);"] # [doc = " ```"] pub struct IntoIter < K , V , A : Allocator = Global > { inner : RawIntoIter < (K , V) , A > , }
};
}
