// Generated macro for VacantEntry (struct)
macro_rules! Depcrate_setVacantEntry {
() => {
// Module: crate::set
// Provides: {"VacantEntry"}
// Dependencies: {}
# [doc = " A view into a vacant entry in a `HashSet`."] # [doc = " It is part of the [`Entry`] enum."] # [doc = ""] # [doc = " [`Entry`]: enum.Entry.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::hash_set::{Entry, HashSet, VacantEntry};"] # [doc = ""] # [doc = " let mut set = HashSet::<&str>::new();"] # [doc = ""] # [doc = " let entry_v: VacantEntry<_, _> = match set.entry(\"a\") {"] # [doc = "     Entry::Vacant(view) => view,"] # [doc = "     Entry::Occupied(_) => unreachable!(),"] # [doc = " };"] # [doc = " entry_v.insert();"] # [doc = " assert!(set.contains(\"a\") && set.len() == 1);"] # [doc = ""] # [doc = " // Nonexistent key (insert)"] # [doc = " match set.entry(\"b\") {"] # [doc = "     Entry::Vacant(view) => { view.insert(); },"] # [doc = "     Entry::Occupied(_) => unreachable!(),"] # [doc = " }"] # [doc = " assert!(set.contains(\"b\") && set.len() == 2);"] # [doc = " ```"] pub struct VacantEntry < 'a , T , S , A : Allocator = Global > { inner : map :: VacantEntry < 'a , T , () , S , A > , }
};
}
