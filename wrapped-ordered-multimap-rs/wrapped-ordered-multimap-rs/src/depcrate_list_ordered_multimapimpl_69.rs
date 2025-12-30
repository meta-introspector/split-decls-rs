// Generated macro for impl_69 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_69 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_69"}
// Dependencies: {}
impl < Key , Value > Debug for IntoIter < Key , Value > where Key : Debug , Value : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . write_str ("IntoIter(") ? ; formatter . debug_list () . entries (self . iter ()) . finish () ? ; formatter . write_str (")") } }
};
}
