// Generated macro for impl_51 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_51 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_51"}
// Dependencies: {}
impl < 'map , Key , Value > EntryValuesDrain < 'map , Key , Value > { # [doc = " Convenience function for creating an empty iterator."] fn empty (values : & 'map mut VecList < ValueEntry < Key , Value > >) -> Self { EntryValuesDrain { head_index : None , remaining : 0 , tail_index : None , values , } } # [doc = " Convenience function for creating a new iterator from a map entry."] fn from_map_entry (values : & 'map mut VecList < ValueEntry < Key , Value > > , map_entry : & MapEntry < Key , Value > ,) -> Self { EntryValuesDrain { head_index : Some (map_entry . head_index) , remaining : map_entry . length , tail_index : Some (map_entry . tail_index) , values , } } # [doc = " Creates an iterator that yields immutable references to all values of a given key."] # [must_use] pub fn iter (& self) -> EntryValues < '_ , Key , Value > { EntryValues { head_index : self . head_index , remaining : self . remaining , tail_index : self . tail_index , values : self . values , } } }
};
}
