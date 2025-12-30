// Generated macro for TowerToHyperService (struct)
macro_rules! Depcrate_service_glueTowerToHyperService {
() => {
// Module: crate::service::glue
// Provides: {"TowerToHyperService"}
// Dependencies: {}
# [doc = " A tower [`Service`][tower-svc] converted into a hyper [`Service`][hyper-svc]."] # [doc = ""] # [doc = " This wraps an inner tower service `S` in a [`hyper::service::Service`] implementation. See"] # [doc = " the module-level documentation of [`service`][crate::service] for more information about using"] # [doc = " [`tower`][tower] services and middleware with [`hyper`]."] # [doc = ""] # [doc = " [hyper-svc]: hyper::service::Service"] # [doc = " [tower]: https://docs.rs/tower/latest/tower/"] # [doc = " [tower-svc]: https://docs.rs/tower/latest/tower/trait.Service.html"] # [derive (Debug , Copy , Clone)] pub struct TowerToHyperService < S > { service : S , }
};
}
