// Generated macro for impl_599 (impl)
macro_rules! Depcrate_service_glueimpl_599 {
() => {
// Module: crate::service::glue
// Provides: {"impl_599"}
// Dependencies: {}
impl < S > TowerToHyperService < S > { # [doc = " Create a new [`TowerToHyperService`] from a tower service."] pub fn new (tower_service : S) -> Self { Self { service : tower_service , } } }
};
}
