// Generated macro for impl_106 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_106 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_106"}
// Dependencies: {}
impl < Key > Debug for Keys < '_ , Key > where Key : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . write_str ("Keys(") ? ; formatter . debug_list () . entries (self . clone ()) . finish () ? ; formatter . write_str (")") } }
};
}
