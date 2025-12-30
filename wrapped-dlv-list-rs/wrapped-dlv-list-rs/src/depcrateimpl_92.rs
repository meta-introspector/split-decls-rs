// Generated macro for impl_92 (impl)
macro_rules! Depcrateimpl_92 {
() => {
// Module: crate
// Provides: {"impl_92"}
// Dependencies: {}
impl < T > Debug for IterMut < '_ , T > where T : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . write_str ("IterMut(") ? ; formatter . debug_list () . entries (self . iter ()) . finish () ? ; formatter . write_str (")") } }
};
}
