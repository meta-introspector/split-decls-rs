// Generated macro for impl_132 (impl)
macro_rules! Depcrate_mapimpl_132 {
() => {
// Module: crate::map
// Provides: {"impl_132"}
// Dependencies: {}
impl < A , F , Req , Res > ServiceFactory < Req > for MapServiceFactory < A , F , Req , Res > where A : ServiceFactory < Req > , F : FnMut (A :: Response) -> Res + Clone , { type Response = Res ; type Error = A :: Error ; type Config = A :: Config ; type Service = Map < A :: Service , F , Req , Res > ; type InitError = A :: InitError ; type Future = MapServiceFuture < A , F , Req , Res > ; fn new_service (& self , cfg : A :: Config) -> Self :: Future { MapServiceFuture :: new (self . a . new_service (cfg) , self . f . clone ()) } }
};
}
