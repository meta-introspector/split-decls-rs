// Generated macro for impl_90 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_90 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_90"}
// Dependencies: {}
impl < Key , Value , State > Debug for KeyValues < '_ , Key , Value , State > where Key : Debug + Eq + Hash , State : BuildHasher , Value : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . write_str ("KeyValues(") ? ; formatter . debug_list () . entries (self . clone ()) . finish () ? ; formatter . write_str (")") } }
};
}
