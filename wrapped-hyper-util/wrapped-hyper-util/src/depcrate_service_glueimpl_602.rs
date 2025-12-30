// Generated macro for impl_602 (impl)
macro_rules! Depcrate_service_glueimpl_602 {
() => {
// Module: crate::service::glue
// Provides: {"impl_602"}
// Dependencies: {}
impl < S , R > Future for TowerToHyperServiceFuture < S , R > where S : tower_service :: Service < R > , { type Output = Result < S :: Response , S :: Error > ; # [inline] fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . project () . future . poll (cx) } }
};
}
