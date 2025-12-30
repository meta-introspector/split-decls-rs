// Generated macro for impl_317 (impl)
macro_rules! Depcrate_hash_mapimpl_317 {
() => {
// Module: crate::hash::map
// Provides: {"impl_317"}
// Dependencies: {}
impl < 'a , K , V , S > Entry < 'a , K , V , S > where K : 'a + Hash + Eq + Clone , V : 'a + Clone , S : 'a + BuildHasher , { # [doc = " Insert the default value provided if there was no value"] # [doc = " already, and return a mutable reference to the value."] pub fn or_insert (self , default : V) -> & 'a mut V { self . or_insert_with (| | default) } # [doc = " Insert the default value from the provided function if there"] # [doc = " was no value already, and return a mutable reference to the"] # [doc = " value."] pub fn or_insert_with < F > (self , default : F) -> & 'a mut V where F : FnOnce () -> V , { match self { Entry :: Occupied (entry) => entry . into_mut () , Entry :: Vacant (entry) => entry . insert (default ()) , } } # [doc = " Insert a default value if there was no value already, and"] # [doc = " return a mutable reference to the value."] pub fn or_default (self) -> & 'a mut V where V : Default , { self . or_insert_with (Default :: default) } # [doc = " Get the key for this entry."] # [must_use] pub fn key (& self) -> & K { match self { Entry :: Occupied (entry) => entry . key () , Entry :: Vacant (entry) => entry . key () , } } # [doc = " Call the provided function to modify the value if the value"] # [doc = " exists."] pub fn and_modify < F > (mut self , f : F) -> Self where F : FnOnce (& mut V) , { match & mut self { Entry :: Occupied (ref mut entry) => f (entry . get_mut ()) , Entry :: Vacant (_) => () , } self } }
};
}
