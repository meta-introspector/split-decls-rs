// Generated macro for impl_120 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_120 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_120"}
// Dependencies: {}
impl < Key , Value > Debug for ValuesMut < '_ , Key , Value > where Key : Debug , Value : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . write_str ("ValuesMut(") ? ; formatter . debug_list () . entries (self . iter ()) . finish () ? ; formatter . write_str (")") } }
};
}
