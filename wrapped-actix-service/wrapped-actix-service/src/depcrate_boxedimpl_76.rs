// Generated macro for impl_76 (impl)
macro_rules! Depcrate_boxedimpl_76 {
() => {
// Module: crate::boxed
// Provides: {"impl_76"}
// Dependencies: {}
impl < C , Req , Res , Err , InitErr > ServiceFactory < Req > for BoxServiceFactory < C , Req , Res , Err , InitErr > where Req : 'static , Res : 'static , Err : 'static , InitErr : 'static , { type Response = Res ; type Error = Err ; type Config = C ; type Service = BoxService < Req , Res , Err > ; type InitError = InitErr ; type Future = BoxFuture < Result < Self :: Service , InitErr > > ; fn new_service (& self , cfg : C) -> Self :: Future { self . 0 . new_service (cfg) } }
};
}
