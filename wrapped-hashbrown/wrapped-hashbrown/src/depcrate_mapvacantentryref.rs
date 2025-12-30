// Generated macro for VacantEntryRef (struct)
macro_rules! Depcrate_mapVacantEntryRef {
() => {
// Module: crate::map
// Provides: {"VacantEntryRef"}
// Dependencies: {}
# [doc = " A view into a vacant entry in a `HashMap`."] # [doc = " It is part of the [`EntryRef`] enum."] # [doc = ""] # [doc = " [`EntryRef`]: enum.EntryRef.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::hash_map::{EntryRef, HashMap, VacantEntryRef};"] # [doc = ""] # [doc = " let mut map = HashMap::<String, i32>::new();"] # [doc = ""] # [doc = " let entry_v: VacantEntryRef<_, _, _, _> = match map.entry_ref(\"a\") {"] # [doc = "     EntryRef::Vacant(view) => view,"] # [doc = "     EntryRef::Occupied(_) => unreachable!(),"] # [doc = " };"] # [doc = " entry_v.insert(10);"] # [doc = " assert!(map[\"a\"] == 10 && map.len() == 1);"] # [doc = ""] # [doc = " // Nonexistent key (insert and update)"] # [doc = " match map.entry_ref(\"b\") {"] # [doc = "     EntryRef::Occupied(_) => unreachable!(),"] # [doc = "     EntryRef::Vacant(view) => {"] # [doc = "         let value = view.insert(2);"] # [doc = "         assert_eq!(*value, 2);"] # [doc = "         *value = 20;"] # [doc = "     }"] # [doc = " }"] # [doc = " assert!(map[\"b\"] == 20 && map.len() == 2);"] # [doc = " ```"] pub struct VacantEntryRef < 'a , 'b , K , Q : ? Sized , V , S , A : Allocator = Global > { hash : u64 , key : & 'b Q , table : & 'a mut HashMap < K , V , S , A > , }
};
}
