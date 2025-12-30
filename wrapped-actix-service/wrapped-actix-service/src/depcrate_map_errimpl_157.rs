// Generated macro for impl_157 (impl)
macro_rules! Depcrate_map_errimpl_157 {
() => {
// Module: crate::map_err
// Provides: {"impl_157"}
// Dependencies: {}
impl < A , Req , F , E > Service < Req > for MapErr < A , Req , F , E > where A : Service < Req > , F : Fn (A :: Error) -> E + Clone , { type Response = A :: Response ; type Error = E ; type Future = MapErrFuture < A , Req , F , E > ; fn poll_ready (& self , ctx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . service . poll_ready (ctx) . map_err (& self . mapper) } fn call (& self , req : Req) -> Self :: Future { MapErrFuture :: new (self . service . call (req) , self . mapper . clone ()) } }
};
}
