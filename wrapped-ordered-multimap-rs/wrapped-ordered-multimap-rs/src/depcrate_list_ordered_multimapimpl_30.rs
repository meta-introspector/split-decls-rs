// Generated macro for impl_30 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_30 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_30"}
// Dependencies: {}
impl < Key , Value > MapEntry < Key , Value > { # [doc = " Convenience function for adding a new value to the entry."] pub fn append (& mut self , index : Index < ValueEntry < Key , Value > >) { self . length += 1 ; self . tail_index = index ; } # [doc = " Convenience function for creating a new multimap entry."] # [must_use] pub fn new (index : Index < ValueEntry < Key , Value > >) -> Self { MapEntry { head_index : index , length : 1 , tail_index : index , } } # [doc = " Convenience function for resetting the entry to contain only one value."] pub fn reset (& mut self , index : Index < ValueEntry < Key , Value > >) { self . head_index = index ; self . length = 1 ; self . tail_index = index ; } }
};
}
