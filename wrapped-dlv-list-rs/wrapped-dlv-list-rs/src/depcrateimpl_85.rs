// Generated macro for impl_85 (impl)
macro_rules! Depcrateimpl_85 {
() => {
// Module: crate
// Provides: {"impl_85"}
// Dependencies: {}
impl < T > Debug for Iter < '_ , T > where T : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . write_str ("Iter(") ? ; formatter . debug_list () . entries (self . clone ()) . finish () ? ; formatter . write_str (")") } }
};
}
