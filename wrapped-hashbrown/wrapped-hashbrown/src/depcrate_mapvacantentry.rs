// Generated macro for VacantEntry (struct)
macro_rules! Depcrate_mapVacantEntry {
() => {
// Module: crate::map
// Provides: {"VacantEntry"}
// Dependencies: {}
# [doc = " A view into a vacant entry in a `HashMap`."] # [doc = " It is part of the [`Entry`] enum."] # [doc = ""] # [doc = " [`Entry`]: enum.Entry.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::hash_map::{Entry, HashMap, VacantEntry};"] # [doc = ""] # [doc = " let mut map = HashMap::<&str, i32>::new();"] # [doc = ""] # [doc = " let entry_v: VacantEntry<_, _, _> = match map.entry(\"a\") {"] # [doc = "     Entry::Vacant(view) => view,"] # [doc = "     Entry::Occupied(_) => unreachable!(),"] # [doc = " };"] # [doc = " entry_v.insert(10);"] # [doc = " assert!(map[&\"a\"] == 10 && map.len() == 1);"] # [doc = ""] # [doc = " // Nonexistent key (insert and update)"] # [doc = " match map.entry(\"b\") {"] # [doc = "     Entry::Occupied(_) => unreachable!(),"] # [doc = "     Entry::Vacant(view) => {"] # [doc = "         let value = view.insert(2);"] # [doc = "         assert_eq!(*value, 2);"] # [doc = "         *value = 20;"] # [doc = "     }"] # [doc = " }"] # [doc = " assert!(map[&\"b\"] == 20 && map.len() == 2);"] # [doc = " ```"] pub struct VacantEntry < 'a , K , V , S = DefaultHashBuilder , A : Allocator = Global > { hash : u64 , key : K , table : & 'a mut HashMap < K , V , S , A > , }
};
}
