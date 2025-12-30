// Generated macro for impl_97 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_97 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_97"}
// Dependencies: {}
impl < Key , Value , State > Debug for KeyValuesMut < '_ , Key , Value , State > where Key : Debug + Eq + Hash , State : BuildHasher , Value : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . write_str ("KeyValuesMut(") ? ; formatter . debug_list () . entries (self . iter ()) . finish () ? ; formatter . write_str (")") } }
};
}
