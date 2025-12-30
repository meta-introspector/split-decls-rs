// Generated macro for impl_65 (impl)
macro_rules! Depcrate_utils_authorimpl_65 {
() => {
// Module: crate::utils::author
// Provides: {"impl_65"}
// Dependencies: {}
impl < T > OptionPat < T > { fn new (opt : Option < T >) -> Self { Self { opt } } fn if_some (& self , f : impl Fn (& T)) { if let Some (t) = & self . opt { f (t) ; } } }
};
}
