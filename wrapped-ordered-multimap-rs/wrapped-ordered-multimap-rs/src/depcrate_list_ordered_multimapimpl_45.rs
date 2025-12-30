// Generated macro for impl_45 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_45 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_45"}
// Dependencies: {}
impl < Key , Value > Debug for EntryValues < '_ , Key , Value > where Value : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . write_str ("EntryValues(") ? ; formatter . debug_list () . entries (self . clone ()) . finish () ? ; formatter . write_str (")") } }
};
}
