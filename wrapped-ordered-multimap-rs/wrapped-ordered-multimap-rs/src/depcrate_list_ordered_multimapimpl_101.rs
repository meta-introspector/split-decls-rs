// Generated macro for impl_101 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_101 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_101"}
// Dependencies: {}
impl < 'map , Key , Value , State > Iterator for KeyValuesMut < 'map , Key , Value , State > where Key : Eq + Hash , State : BuildHasher , { type Item = (& 'map Key , EntryValuesMut < 'map , Key , Value >) ; fn next (& mut self) -> Option < Self :: Item > { let key = self . iter . next () ? ; let hash = self . build_hasher . hash_one (key) ; let (_ , map_entry) = raw_entry (self . keys , self . map , hash , key) . unwrap () ; let iter = EntryValuesMut :: from_map_entry (unsafe { & mut * self . values } , map_entry) ; Some ((key , iter)) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
