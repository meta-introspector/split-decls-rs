// Generated macro for impl_60 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_60 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_60"}
// Dependencies: {}
impl < Key , Value > Debug for EntryValuesMut < '_ , Key , Value > where Value : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . write_str ("EntryValuesMut(") ? ; formatter . debug_list () . entries (self . iter ()) . finish () ? ; formatter . write_str (")") } }
};
}
