// Generated macro for impl_17 (impl)
macro_rules! Depcrate_dictionaryimpl_17 {
() => {
// Module: crate::dictionary
// Provides: {"impl_17"}
// Dependencies: {}
# [cfg (any (test , feature = "enable_unstable_features_that_may_break_with_minor_version_bumps"))] impl < 'a > VacantEntry < 'a > { # [doc = " Gets a reference to the key that would be used when inserting a value through the"] # [doc = " VacantEntry."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use plist::dictionary::Entry;"] # [doc = ""] # [doc = " let mut dict = plist::Dictionary::new();"] # [doc = ""] # [doc = " match dict.entry(\"serde\") {"] # [doc = "     Entry::Vacant(vacant) => assert_eq!(vacant.key(), &\"serde\"),"] # [doc = "     Entry::Occupied(_) => unimplemented!(),"] # [doc = " }"] # [doc = " ```"] # [inline] pub fn key (& self) -> & String { self . vacant . key () } # [doc = " Sets the value of the entry with the VacantEntry's key, and returns a mutable reference"] # [doc = " to it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use plist::dictionary::Entry;"] # [doc = ""] # [doc = " let mut dict = plist::Dictionary::new();"] # [doc = ""] # [doc = " match dict.entry(\"serde\") {"] # [doc = "     Entry::Vacant(vacant) => vacant.insert(\"hoho\".into()),"] # [doc = "     Entry::Occupied(_) => unimplemented!(),"] # [doc = " };"] # [doc = " ```"] # [inline] pub fn insert (self , value : Value) -> & 'a mut Value { self . vacant . insert (value) } }
};
}
