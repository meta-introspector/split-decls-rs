// Generated macro for impl_32 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_32 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_32"}
// Dependencies: {}
impl < Key , Value > ValueEntry < Key , Value > { # [doc = " Convenience function for creating a new value entry."] # [must_use] pub fn new (key_index : Index < Key > , value : Value) -> Self { ValueEntry { key_index , next_index : None , previous_index : None , value , } } }
};
}
