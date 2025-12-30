// Generated macro for impl_94 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_94 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_94"}
// Dependencies: {}
impl < 'map , Key , Value , State > Iterator for KeyValues < 'map , Key , Value , State > where Key : Eq + Hash , State : BuildHasher , { type Item = (& 'map Key , EntryValues < 'map , Key , Value >) ; fn next (& mut self) -> Option < Self :: Item > { let key = self . iter . next () ? ; let hash = self . build_hasher . hash_one (key) ; let (_ , map_entry) = raw_entry (self . keys , self . map , hash , key) . unwrap () ; let iter = EntryValues :: from_map_entry (self . values , map_entry) ; Some ((key , iter)) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
