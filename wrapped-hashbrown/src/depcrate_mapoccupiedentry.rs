// Generated macro for OccupiedEntry (struct)
macro_rules! Depcrate_mapOccupiedEntry {
() => {
// Module: crate::map
// Provides: {"OccupiedEntry"}
// Dependencies: {}
# [doc = " A view into an occupied entry in a [`HashMap`]."] # [doc = " It is part of the [`Entry`] and [`EntryRef`] enums."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::hash_map::{Entry, HashMap, OccupiedEntry};"] # [doc = ""] # [doc = " let mut map = HashMap::new();"] # [doc = " map.extend([(\"a\", 10), (\"b\", 20), (\"c\", 30)]);"] # [doc = ""] # [doc = " let _entry_o: OccupiedEntry<_, _, _> = map.entry(\"a\").insert(100);"] # [doc = " assert_eq!(map.len(), 3);"] # [doc = ""] # [doc = " // Existing key (insert and update)"] # [doc = " match map.entry(\"a\") {"] # [doc = "     Entry::Vacant(_) => unreachable!(),"] # [doc = "     Entry::Occupied(mut view) => {"] # [doc = "         assert_eq!(view.get(), &100);"] # [doc = "         let v = view.get_mut();"] # [doc = "         *v *= 10;"] # [doc = "         assert_eq!(view.insert(1111), 1000);"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(map[&\"a\"], 1111);"] # [doc = " assert_eq!(map.len(), 3);"] # [doc = ""] # [doc = " // Existing key (take)"] # [doc = " match map.entry(\"c\") {"] # [doc = "     Entry::Vacant(_) => unreachable!(),"] # [doc = "     Entry::Occupied(view) => {"] # [doc = "         assert_eq!(view.remove_entry(), (\"c\", 30));"] # [doc = "     }"] # [doc = " }"] # [doc = " assert_eq!(map.get(&\"c\"), None);"] # [doc = " assert_eq!(map.len(), 2);"] # [doc = " ```"] pub struct OccupiedEntry < 'a , K , V , S = DefaultHashBuilder , A : Allocator = Global > { hash : u64 , elem : Bucket < (K , V) > , table : & 'a mut HashMap < K , V , S , A > , }
};
}
