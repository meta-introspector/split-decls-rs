// Generated macro for impl_165 (impl)
macro_rules! Depcrate_coercionimpl_165 {
() => {
// Module: crate::coercion
// Provides: {"impl_165"}
// Dependencies: {}
impl < 'a , T > AsCoercionSite for & 'a T where T : AsCoercionSite , { fn as_coercion_site (& self) -> & hir :: Expr < '_ > { (* * self) . as_coercion_site () } }
};
}
