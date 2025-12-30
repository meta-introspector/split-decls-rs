// Generated macro for impl_78 (impl)
macro_rules! Depcrateimpl_78 {
() => {
// Module: crate
// Provides: {"impl_78"}
// Dependencies: {}
impl < T > Debug for IntoIter < T > where T : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . write_str ("IntoIter(") ? ; formatter . debug_list () . entries (self . iter ()) . finish () ? ; formatter . write_str (")") } }
};
}
