// Generated macro for macro_601 (macro)
macro_rules! Depcrate_service_gluemacro_601 {
() => {
// Module: crate::service::glue
// Provides: {"macro_601"}
// Dependencies: {}
pin_project ! { # [doc = " Response future for [`TowerToHyperService`]."] # [doc = ""] # [doc = " This future is acquired by [`call`][hyper::service::Service::call]ing a"] # [doc = " [`TowerToHyperService`]."] pub struct TowerToHyperServiceFuture < S , R > where S : tower_service :: Service < R >, { # [pin] future : Oneshot < S , R >, } }
};
}
