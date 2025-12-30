// Generated macro for impl_98 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_98 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_98"}
// Dependencies: {}
impl < Key , Value , State > DoubleEndedIterator for KeyValuesMut < '_ , Key , Value , State > where Key : Eq + Hash , State : BuildHasher , { fn next_back (& mut self) -> Option < Self :: Item > { let key = self . iter . next_back () ? ; let hash = self . build_hasher . hash_one (key) ; let (_ , map_entry) = raw_entry (self . keys , self . map , hash , key) . unwrap () ; let iter = EntryValuesMut :: from_map_entry (unsafe { & mut * self . values } , map_entry) ; Some ((key , iter)) } }
};
}
