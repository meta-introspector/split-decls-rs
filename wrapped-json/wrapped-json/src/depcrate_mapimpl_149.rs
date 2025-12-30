// Generated macro for impl_149 (impl)
macro_rules! Depcrate_mapimpl_149 {
() => {
// Module: crate::map
// Provides: {"impl_149"}
// Dependencies: {}
impl < 'a > VacantEntry < 'a > { # [doc = " Gets a reference to the key that would be used when inserting a value"] # [doc = " through the VacantEntry."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::map::Entry;"] # [doc = ""] # [doc = " let mut map = serde_json::Map::new();"] # [doc = ""] # [doc = " match map.entry(\"serde\") {"] # [doc = "     Entry::Vacant(vacant) => {"] # [doc = "         assert_eq!(vacant.key(), &\"serde\");"] # [doc = "     }"] # [doc = "     Entry::Occupied(_) => unimplemented!(),"] # [doc = " }"] # [doc = " ```"] # [inline] pub fn key (& self) -> & String { self . vacant . key () } # [doc = " Sets the value of the entry with the VacantEntry's key, and returns a"] # [doc = " mutable reference to it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use serde_json::json;"] # [doc = " #"] # [doc = " use serde_json::map::Entry;"] # [doc = ""] # [doc = " let mut map = serde_json::Map::new();"] # [doc = ""] # [doc = " match map.entry(\"serde\") {"] # [doc = "     Entry::Vacant(vacant) => {"] # [doc = "         vacant.insert(json!(\"hoho\"));"] # [doc = "     }"] # [doc = "     Entry::Occupied(_) => unimplemented!(),"] # [doc = " }"] # [doc = " ```"] # [inline] pub fn insert (self , value : Value) -> & 'a mut Value { self . vacant . insert (value) } }
};
}
