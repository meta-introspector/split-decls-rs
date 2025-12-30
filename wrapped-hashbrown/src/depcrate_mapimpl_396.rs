// Generated macro for impl_396 (impl)
macro_rules! Depcrate_mapimpl_396 {
() => {
// Module: crate::map
// Provides: {"impl_396"}
// Dependencies: {}
impl < 'a , K , V : Default , S , A : Allocator > Entry < 'a , K , V , S , A > { # [doc = " Ensures a value is in the entry by inserting the default value if empty,"] # [doc = " and returns a mutable reference to the value in the entry."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = ""] # [doc = " let mut map: HashMap<&str, Option<u32>> = HashMap::new();"] # [doc = ""] # [doc = " // nonexistent key"] # [doc = " map.entry(\"poneyland\").or_default();"] # [doc = " assert_eq!(map[\"poneyland\"], None);"] # [doc = ""] # [doc = " map.insert(\"horseland\", Some(3));"] # [doc = ""] # [doc = " // existing key"] # [doc = " assert_eq!(map.entry(\"horseland\").or_default(), &mut Some(3));"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] pub fn or_default (self) -> & 'a mut V where K : Hash , S : BuildHasher , { match self { Entry :: Occupied (entry) => entry . into_mut () , Entry :: Vacant (entry) => entry . insert (Default :: default ()) , } } }
};
}
