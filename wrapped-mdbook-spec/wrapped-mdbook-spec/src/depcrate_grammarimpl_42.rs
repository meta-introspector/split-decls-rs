// Generated macro for impl_42 (impl)
macro_rules! Depcrate_grammarimpl_42 {
() => {
// Module: crate::grammar
// Provides: {"impl_42"}
// Dependencies: {}
impl Grammar { fn visit_nt (& self , callback : & mut dyn FnMut (& str)) { for p in self . productions . values () { p . expression . visit_nt (callback) ; } } }
};
}
