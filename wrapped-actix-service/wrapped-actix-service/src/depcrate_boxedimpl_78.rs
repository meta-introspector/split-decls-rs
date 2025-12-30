// Generated macro for impl_78 (impl)
macro_rules! Depcrate_boxedimpl_78 {
() => {
// Module: crate::boxed
// Provides: {"impl_78"}
// Dependencies: {}
impl < SF , Req , Cfg , Res , Err , InitErr > ServiceFactory < Req > for FactoryWrapper < SF > where Req : 'static , Res : 'static , Err : 'static , InitErr : 'static , SF : ServiceFactory < Req , Config = Cfg , Response = Res , Error = Err , InitError = InitErr > , SF :: Future : 'static , SF :: Service : 'static , < SF :: Service as Service < Req > > :: Future : 'static , { type Response = Res ; type Error = Err ; type Config = Cfg ; type Service = BoxService < Req , Res , Err > ; type InitError = InitErr ; type Future = BoxFuture < Result < Self :: Service , Self :: InitError > > ; fn new_service (& self , cfg : Cfg) -> Self :: Future { let f = self . 0 . new_service (cfg) ; Box :: pin (async { f . await . map (| s | Box :: new (ServiceWrapper :: new (s)) as _) }) } }
};
}
