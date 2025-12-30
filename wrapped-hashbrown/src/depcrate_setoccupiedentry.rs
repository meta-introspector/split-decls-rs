// Generated macro for OccupiedEntry (struct)
macro_rules! Depcrate_setOccupiedEntry {
() => {
// Module: crate::set
// Provides: {"OccupiedEntry"}
// Dependencies: {}
# [doc = " A view into an occupied entry in a `HashSet`."] # [doc = " It is part of the [`Entry`] enum."] # [doc = ""] # [doc = " [`Entry`]: enum.Entry.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::hash_set::{Entry, HashSet, OccupiedEntry};"] # [doc = ""] # [doc = " let mut set = HashSet::new();"] # [doc = " set.extend([\"a\", \"b\", \"c\"]);"] # [doc = ""] # [doc = " let _entry_o: OccupiedEntry<_, _> = set.entry(\"a\").insert();"] # [doc = " assert_eq!(set.len(), 3);"] # [doc = ""] # [doc = " // Existing key"] # [doc = " match set.entry(\"a\") {"] # [doc = "     Entry::Vacant(_) => unreachable!(),"] # [doc = "     Entry::Occupied(view) => {"] # [doc = "         assert_eq!(view.get(), &\"a\");"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(set.len(), 3);"] # [doc = ""] # [doc = " // Existing key (take)"] # [doc = " match set.entry(\"c\") {"] # [doc = "     Entry::Vacant(_) => unreachable!(),"] # [doc = "     Entry::Occupied(view) => {"] # [doc = "         assert_eq!(view.remove(), \"c\");"] # [doc = "     }"] # [doc = " }"] # [doc = " assert_eq!(set.get(&\"c\"), None);"] # [doc = " assert_eq!(set.len(), 2);"] # [doc = " ```"] pub struct OccupiedEntry < 'a , T , S , A : Allocator = Global > { inner : map :: OccupiedEntry < 'a , T , () , S , A > , }
};
}
