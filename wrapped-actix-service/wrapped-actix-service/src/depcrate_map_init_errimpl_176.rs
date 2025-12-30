// Generated macro for impl_176 (impl)
macro_rules! Depcrate_map_init_errimpl_176 {
() => {
// Module: crate::map_init_err
// Provides: {"impl_176"}
// Dependencies: {}
impl < A , F , Req , E > ServiceFactory < Req > for MapInitErr < A , F , Req , E > where A : ServiceFactory < Req > , F : Fn (A :: InitError) -> E + Clone , { type Response = A :: Response ; type Error = A :: Error ; type Config = A :: Config ; type Service = A :: Service ; type InitError = E ; type Future = MapInitErrFuture < A , F , Req , E > ; fn new_service (& self , cfg : A :: Config) -> Self :: Future { MapInitErrFuture :: new (self . a . new_service (cfg) , self . f . clone ()) } }
};
}
