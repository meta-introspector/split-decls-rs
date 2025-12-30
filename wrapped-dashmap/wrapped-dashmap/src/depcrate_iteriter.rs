// Generated macro for Iter (struct)
macro_rules! Depcrate_iterIter {
() => {
// Module: crate::iter
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " Iterator over a DashMap yielding immutable references."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use dashmap::DashMap;"] # [doc = ""] # [doc = " let map = DashMap::new();"] # [doc = " map.insert(\"hello\", \"world\");"] # [doc = " assert_eq!(map.iter().count(), 1);"] # [doc = " ```"] pub struct Iter < 'a , K , V > { shards : std :: slice :: Iter < 'a , CachePadded < RwLock < HashMap < K , V > > > > , current : Option < GuardIter < 'a , K , V > > , }
};
}
