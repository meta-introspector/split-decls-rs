// Generated macro for impl_179 (impl)
macro_rules! Depcrate_serimpl_179 {
() => {
// Module: crate::ser
// Provides: {"impl_179"}
// Dependencies: {}
impl < 'a , W : fmt :: Write > Drop for Compound < 'a , W > { fn drop (& mut self) { if let Some (limit) = & mut self . ser . recursion_limit { * limit = limit . saturating_add (1) ; } } }
};
}
