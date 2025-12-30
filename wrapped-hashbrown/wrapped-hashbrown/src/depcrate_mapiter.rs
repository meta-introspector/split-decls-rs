// Generated macro for Iter (struct)
macro_rules! Depcrate_mapIter {
() => {
// Module: crate::map
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the entries of a `HashMap` in arbitrary order."] # [doc = " The iterator element type is `(&'a K, &'a V)`."] # [doc = ""] # [doc = " This `struct` is created by the [`iter`] method on [`HashMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`iter`]: struct.HashMap.html#method.iter"] # [doc = " [`HashMap`]: struct.HashMap.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = ""] # [doc = " let map: HashMap<_, _> = [(1, \"a\"), (2, \"b\"), (3, \"c\")].into();"] # [doc = ""] # [doc = " let mut iter = map.iter();"] # [doc = " let mut vec = vec![iter.next(), iter.next(), iter.next()];"] # [doc = ""] # [doc = " // The `Iter` iterator produces items in arbitrary order, so the"] # [doc = " // items must be sorted to test them against a sorted array."] # [doc = " vec.sort_unstable();"] # [doc = " assert_eq!(vec, [Some((&1, &\"a\")), Some((&2, &\"b\")), Some((&3, &\"c\"))]);"] # [doc = ""] # [doc = " // It is fused iterator"] # [doc = " assert_eq!(iter.next(), None);"] # [doc = " assert_eq!(iter.next(), None);"] # [doc = " ```"] pub struct Iter < 'a , K , V > { inner : RawIter < (K , V) > , marker : PhantomData < (& 'a K , & 'a V) > , }
};
}
