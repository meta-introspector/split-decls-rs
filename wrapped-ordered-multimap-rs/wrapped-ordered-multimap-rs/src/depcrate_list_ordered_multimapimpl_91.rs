// Generated macro for impl_91 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_91 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_91"}
// Dependencies: {}
impl < Key , Value , State > DoubleEndedIterator for KeyValues < '_ , Key , Value , State > where Key : Eq + Hash , State : BuildHasher , { fn next_back (& mut self) -> Option < Self :: Item > { let key = self . iter . next_back () ? ; let hash = self . build_hasher . hash_one (key) ; let (_ , map_entry) = raw_entry (self . keys , self . map , hash , key) . unwrap () ; let iter = EntryValues :: from_map_entry (self . values , map_entry) ; Some ((key , iter)) } }
};
}
