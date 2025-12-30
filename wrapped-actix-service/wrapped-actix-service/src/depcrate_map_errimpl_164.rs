// Generated macro for impl_164 (impl)
macro_rules! Depcrate_map_errimpl_164 {
() => {
// Module: crate::map_err
// Provides: {"impl_164"}
// Dependencies: {}
impl < SF , Req , F , E > ServiceFactory < Req > for MapErrServiceFactory < SF , Req , F , E > where SF : ServiceFactory < Req > , F : Fn (SF :: Error) -> E + Clone , { type Response = SF :: Response ; type Error = E ; type Config = SF :: Config ; type Service = MapErr < SF :: Service , Req , F , E > ; type InitError = SF :: InitError ; type Future = MapErrServiceFuture < SF , Req , F , E > ; fn new_service (& self , cfg : SF :: Config) -> Self :: Future { MapErrServiceFuture :: new (self . a . new_service (cfg) , self . f . clone ()) } }
};
}
