// Generated macro for impl_217 (impl)
macro_rules! Depcrate_thenimpl_217 {
() => {
// Module: crate::then
// Provides: {"impl_217"}
// Dependencies: {}
impl < A , B , Req > ServiceFactory < Req > for ThenServiceFactory < A , B , Req > where A : ServiceFactory < Req > , A :: Config : Clone , B : ServiceFactory < Result < A :: Response , A :: Error > , Config = A :: Config , Error = A :: Error , InitError = A :: InitError , > , { type Response = B :: Response ; type Error = A :: Error ; type Config = A :: Config ; type Service = ThenService < A :: Service , B :: Service , Req > ; type InitError = A :: InitError ; type Future = ThenServiceFactoryResponse < A , B , Req > ; fn new_service (& self , cfg : A :: Config) -> Self :: Future { let srv = & * self . 0 ; ThenServiceFactoryResponse :: new (srv . 0 . new_service (cfg . clone ()) , srv . 1 . new_service (cfg)) } }
};
}
