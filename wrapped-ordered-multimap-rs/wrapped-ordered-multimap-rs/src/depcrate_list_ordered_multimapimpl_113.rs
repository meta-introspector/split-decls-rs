// Generated macro for impl_113 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_113 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_113"}
// Dependencies: {}
impl < Key , Value > Debug for Values < '_ , Key , Value > where Key : Debug , Value : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . write_str ("Values(") ? ; formatter . debug_list () . entries (self . clone ()) . finish () ? ; formatter . write_str (")") } }
};
}
