// Generated macro for impl_69 (impl)
macro_rules! Depcrate_utils_authorimpl_69 {
() => {
// Module: crate::utils::author
// Provides: {"impl_69"}
// Dependencies: {}
impl < T > OptionPat < T > { fn new (opt : Option < T >) -> Self { Self { opt } } fn if_some (& self , f : impl Fn (& T)) { if let Some (t) = & self . opt { f (t) ; } } }
};
}
