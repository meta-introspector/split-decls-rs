// Generated macro for impl_17 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_17 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_17"}
// Dependencies: {}
impl < Key , Value , State > Debug for ListOrderedMultimap < Key , Value , State > where Key : Debug , Value : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . debug_map () . entries (self . iter ()) . finish () } }
};
}
