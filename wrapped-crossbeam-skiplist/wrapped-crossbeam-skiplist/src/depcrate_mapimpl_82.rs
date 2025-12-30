// Generated macro for impl_82 (impl)
macro_rules! Depcrate_mapimpl_82 {
() => {
// Module: crate::map
// Provides: {"impl_82"}
// Dependencies: {}
impl < K , V > SkipMap < K , V > { # [doc = " Returns a new, empty map."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_skiplist::SkipMap;"] # [doc = ""] # [doc = " let map: SkipMap<i32, &str> = SkipMap::new();"] # [doc = " ```"] pub fn new () -> Self { Self { inner : base :: SkipList :: new (epoch :: default_collector () . clone ()) , } } # [doc = " Returns `true` if the map is empty."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use crossbeam_skiplist::SkipMap;"] # [doc = ""] # [doc = " let map: SkipMap<&str, &str> = SkipMap::new();"] # [doc = " assert!(map.is_empty());"] # [doc = ""] # [doc = " map.insert(\"key\", \"value\");"] # [doc = " assert!(!map.is_empty());"] # [doc = " ```"] pub fn is_empty (& self) -> bool { self . inner . is_empty () } # [doc = " Returns the number of entries in the map."] # [doc = ""] # [doc = " If the map is being concurrently modified, consider the returned number just an"] # [doc = " approximation without any guarantees."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use crossbeam_skiplist::SkipMap;"] # [doc = ""] # [doc = " let map = SkipMap::new();"] # [doc = " map.insert(0, 1);"] # [doc = " assert_eq!(map.len(), 1);"] # [doc = ""] # [doc = " for x in 1..=5 {"] # [doc = "     map.insert(x, x + 1);"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(map.len(), 6);"] # [doc = " ```"] pub fn len (& self) -> usize { self . inner . len () } }
};
}
