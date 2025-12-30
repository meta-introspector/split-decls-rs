// Generated macro for IntoKeys (struct)
macro_rules! Depcrate_mapIntoKeys {
() => {
// Module: crate::map
// Provides: {"IntoKeys"}
// Dependencies: {}
# [doc = " An owning iterator over the keys of a `HashMap` in arbitrary order."] # [doc = " The iterator element type is `K`."] # [doc = ""] # [doc = " This `struct` is created by the [`into_keys`] method on [`HashMap`]."] # [doc = " See its documentation for more."] # [doc = " The map cannot be used after calling that method."] # [doc = ""] # [doc = " [`into_keys`]: struct.HashMap.html#method.into_keys"] # [doc = " [`HashMap`]: struct.HashMap.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = ""] # [doc = " let map: HashMap<_, _> = [(1, \"a\"), (2, \"b\"), (3, \"c\")].into();"] # [doc = ""] # [doc = " let mut keys = map.into_keys();"] # [doc = " let mut vec = vec![keys.next(), keys.next(), keys.next()];"] # [doc = ""] # [doc = " // The `IntoKeys` iterator produces keys in arbitrary order, so the"] # [doc = " // keys must be sorted to test them against a sorted array."] # [doc = " vec.sort_unstable();"] # [doc = " assert_eq!(vec, [Some(1), Some(2), Some(3)]);"] # [doc = ""] # [doc = " // It is fused iterator"] # [doc = " assert_eq!(keys.next(), None);"] # [doc = " assert_eq!(keys.next(), None);"] # [doc = " ```"] pub struct IntoKeys < K , V , A : Allocator = Global > { inner : IntoIter < K , V , A > , }
};
}
