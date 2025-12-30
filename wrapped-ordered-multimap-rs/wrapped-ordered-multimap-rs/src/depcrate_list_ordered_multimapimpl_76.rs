// Generated macro for impl_76 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_76 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_76"}
// Dependencies: {}
impl < Key , Value > Debug for Iter < '_ , Key , Value > where Key : Debug , Value : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . write_str ("Iter(") ? ; formatter . debug_list () . entries (self . clone ()) . finish () ? ; formatter . write_str (")") } }
};
}
