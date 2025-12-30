// Generated macro for impl_52 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_52 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_52"}
// Dependencies: {}
impl < Key , Value > Debug for EntryValuesDrain < '_ , Key , Value > where Key : Debug , Value : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . write_str ("EntryValuesDrain(") ? ; formatter . debug_list () . entries (self . iter ()) . finish () ? ; formatter . write_str (")") } }
};
}
