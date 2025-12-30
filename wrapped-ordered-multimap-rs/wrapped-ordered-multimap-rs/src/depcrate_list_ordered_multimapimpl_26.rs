// Generated macro for impl_26 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_26 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_26"}
// Dependencies: {}
impl < Key , Value , State > PartialEq for ListOrderedMultimap < Key , Value , State > where Key : PartialEq , Value : PartialEq , { fn eq (& self , other : & ListOrderedMultimap < Key , Value , State >) -> bool { if self . keys_len () != other . keys_len () || self . values_len () != other . values_len () { return false ; } self . iter () . eq (other . iter ()) } }
};
}
