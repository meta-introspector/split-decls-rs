// Generated macro for OwningIter (struct)
macro_rules! Depcrate_iterOwningIter {
() => {
// Module: crate::iter
// Provides: {"OwningIter"}
// Dependencies: {}
# [doc = " Iterator over a DashMap yielding key value pairs."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use dashmap::DashMap;"] # [doc = ""] # [doc = " let map = DashMap::new();"] # [doc = " map.insert(\"hello\", \"world\");"] # [doc = " map.insert(\"alex\", \"steve\");"] # [doc = " let pairs: Vec<(&'static str, &'static str)> = map.into_iter().collect();"] # [doc = " assert_eq!(pairs.len(), 2);"] # [doc = " ```"] pub struct OwningIter < K , V > { shards : std :: vec :: IntoIter < CachePadded < RwLock < HashMap < K , V > > > > , current : Option < GuardOwningIter < K , V > > , }
};
}
