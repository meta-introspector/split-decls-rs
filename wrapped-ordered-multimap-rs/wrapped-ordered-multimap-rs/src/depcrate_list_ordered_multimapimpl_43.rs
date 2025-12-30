// Generated macro for impl_43 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_43 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_43"}
// Dependencies: {}
impl < 'map , Key , Value > EntryValues < 'map , Key , Value > { # [doc = " Convenience function for creating an empty iterator."] # [must_use] fn empty (values : & 'map VecList < ValueEntry < Key , Value > >) -> Self { EntryValues { head_index : None , remaining : 0 , tail_index : None , values , } } # [doc = " Convenience function for creating a new iterator from a map entry."] # [must_use] fn from_map_entry (values : & 'map VecList < ValueEntry < Key , Value > > , map_entry : & MapEntry < Key , Value > ,) -> Self { EntryValues { head_index : Some (map_entry . head_index) , remaining : map_entry . length , tail_index : Some (map_entry . tail_index) , values , } } }
};
}
