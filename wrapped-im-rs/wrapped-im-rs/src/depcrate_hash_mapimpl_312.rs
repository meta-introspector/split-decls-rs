// Generated macro for impl_312 (impl)
macro_rules! Depcrate_hash_mapimpl_312 {
() => {
// Module: crate::hash::map
// Provides: {"impl_312"}
// Dependencies: {}
impl < K , V > HashMap < K , V , RandomState > where K : Hash + Eq + Clone , V : Clone , { # [doc = " Construct a hash map with a single mapping."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate im;"] # [doc = " # use im::hashmap::HashMap;"] # [doc = " let map = HashMap::unit(123, \"onetwothree\");"] # [doc = " assert_eq!("] # [doc = "   map.get(&123),"] # [doc = "   Some(&\"onetwothree\")"] # [doc = " );"] # [doc = " ```"] # [inline] # [must_use] pub fn unit (k : K , v : V) -> HashMap < K , V > { HashMap :: new () . update (k , v) } }
};
}
