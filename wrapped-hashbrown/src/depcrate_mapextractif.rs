// Generated macro for ExtractIf (struct)
macro_rules! Depcrate_mapExtractIf {
() => {
// Module: crate::map
// Provides: {"ExtractIf"}
// Dependencies: {}
# [doc = " A draining iterator over entries of a `HashMap` which don't satisfy the predicate"] # [doc = " `f(&k, &mut v)` in arbitrary order. The iterator element type is `(K, V)`."] # [doc = ""] # [doc = " This `struct` is created by the [`extract_if`] method on [`HashMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`extract_if`]: struct.HashMap.html#method.extract_if"] # [doc = " [`HashMap`]: struct.HashMap.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = ""] # [doc = " let mut map: HashMap<i32, &str> = [(1, \"a\"), (2, \"b\"), (3, \"c\")].into();"] # [doc = ""] # [doc = " let mut extract_if = map.extract_if(|k, _v| k % 2 != 0);"] # [doc = " let mut vec = vec![extract_if.next(), extract_if.next()];"] # [doc = ""] # [doc = " // The `ExtractIf` iterator produces items in arbitrary order, so the"] # [doc = " // items must be sorted to test them against a sorted array."] # [doc = " vec.sort_unstable();"] # [doc = " assert_eq!(vec, [Some((1, \"a\")),Some((3, \"c\"))]);"] # [doc = ""] # [doc = " // It is fused iterator"] # [doc = " assert_eq!(extract_if.next(), None);"] # [doc = " assert_eq!(extract_if.next(), None);"] # [doc = " drop(extract_if);"] # [doc = ""] # [doc = " assert_eq!(map.len(), 1);"] # [doc = " ```"] # [must_use = "Iterators are lazy unless consumed"] pub struct ExtractIf < 'a , K , V , F , A : Allocator = Global > { f : F , inner : RawExtractIf < 'a , (K , V) , A > , }
};
}
