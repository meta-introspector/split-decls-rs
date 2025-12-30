// Generated macro for impl_600 (impl)
macro_rules! Depcrate_service_glueimpl_600 {
() => {
// Module: crate::service::glue
// Provides: {"impl_600"}
// Dependencies: {}
impl < S , R > hyper :: service :: Service < R > for TowerToHyperService < S > where S : tower_service :: Service < R > + Clone , { type Response = S :: Response ; type Error = S :: Error ; type Future = TowerToHyperServiceFuture < S , R > ; fn call (& self , req : R) -> Self :: Future { TowerToHyperServiceFuture { future : Oneshot :: new (self . service . clone () , req) , } } }
};
}
