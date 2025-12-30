// Generated macro for impl_83 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_83 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_83"}
// Dependencies: {}
impl < Key , Value > Debug for IterMut < '_ , Key , Value > where Key : Debug , Value : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . write_str ("IterMut(") ? ; formatter . debug_list () . entries (self . iter ()) . finish () ? ; formatter . write_str (")") } }
};
}
