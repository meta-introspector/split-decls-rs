// Generated macro for Keys (struct)
macro_rules! Depcrate_mapKeys {
() => {
// Module: crate::map
// Provides: {"Keys"}
// Dependencies: {}
# [doc = " An iterator over the keys of a `HashMap` in arbitrary order."] # [doc = " The iterator element type is `&'a K`."] # [doc = ""] # [doc = " This `struct` is created by the [`keys`] method on [`HashMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`keys`]: struct.HashMap.html#method.keys"] # [doc = " [`HashMap`]: struct.HashMap.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = ""] # [doc = " let map: HashMap<_, _> = [(1, \"a\"), (2, \"b\"), (3, \"c\")].into();"] # [doc = ""] # [doc = " let mut keys = map.keys();"] # [doc = " let mut vec = vec![keys.next(), keys.next(), keys.next()];"] # [doc = ""] # [doc = " // The `Keys` iterator produces keys in arbitrary order, so the"] # [doc = " // keys must be sorted to test them against a sorted array."] # [doc = " vec.sort_unstable();"] # [doc = " assert_eq!(vec, [Some(&1), Some(&2), Some(&3)]);"] # [doc = ""] # [doc = " // It is fused iterator"] # [doc = " assert_eq!(keys.next(), None);"] # [doc = " assert_eq!(keys.next(), None);"] # [doc = " ```"] pub struct Keys < 'a , K , V > { inner : Iter < 'a , K , V > , }
};
}
