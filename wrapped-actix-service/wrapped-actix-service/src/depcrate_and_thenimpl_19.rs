// Generated macro for impl_19 (impl)
macro_rules! Depcrate_and_thenimpl_19 {
() => {
// Module: crate::and_then
// Provides: {"impl_19"}
// Dependencies: {}
impl < A , B , Req > ServiceFactory < Req > for AndThenServiceFactory < A , B , Req > where A : ServiceFactory < Req > , A :: Config : Clone , B : ServiceFactory < A :: Response , Config = A :: Config , Error = A :: Error , InitError = A :: InitError > , { type Response = B :: Response ; type Error = A :: Error ; type Config = A :: Config ; type Service = AndThenService < A :: Service , B :: Service , Req > ; type InitError = A :: InitError ; type Future = AndThenServiceFactoryResponse < A , B , Req > ; fn new_service (& self , cfg : A :: Config) -> Self :: Future { let inner = & * self . inner ; AndThenServiceFactoryResponse :: new (inner . 0 . new_service (cfg . clone ()) , inner . 1 . new_service (cfg) ,) } }
};
}
