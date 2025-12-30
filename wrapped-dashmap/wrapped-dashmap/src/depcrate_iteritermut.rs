// Generated macro for IterMut (struct)
macro_rules! Depcrate_iterIterMut {
() => {
// Module: crate::iter
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " Iterator over a DashMap yielding mutable references."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use dashmap::DashMap;"] # [doc = ""] # [doc = " let map = DashMap::new();"] # [doc = " map.insert(\"Johnny\", 21);"] # [doc = " map.iter_mut().for_each(|mut r| *r += 1);"] # [doc = " assert_eq!(*map.get(\"Johnny\").unwrap(), 22);"] # [doc = " ```"] pub struct IterMut < 'a , K , V > { shards : std :: slice :: Iter < 'a , CachePadded < RwLock < HashMap < K , V > > > > , current : Option < GuardIterMut < 'a , K , V > > , }
};
}
