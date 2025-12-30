// Generated macro for impl_71 (impl)
macro_rules! Depcrateimpl_71 {
() => {
// Module: crate
// Provides: {"impl_71"}
// Dependencies: {}
impl < T > Debug for Indices < '_ , T > where T : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . write_str ("Indices(") ? ; formatter . debug_list () . entries (self . clone ()) . finish () ? ; formatter . write_str (")") } }
};
}
