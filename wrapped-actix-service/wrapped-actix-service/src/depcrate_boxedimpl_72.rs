// Generated macro for impl_72 (impl)
macro_rules! Depcrate_boxedimpl_72 {
() => {
// Module: crate::boxed
// Provides: {"impl_72"}
// Dependencies: {}
impl < S , Req , Res , Err > Service < Req > for ServiceWrapper < S > where S : Service < Req , Response = Res , Error = Err > , S :: Future : 'static , { type Response = Res ; type Error = Err ; type Future = BoxFuture < Result < Res , Err > > ; crate :: forward_ready ! (inner) ; fn call (& self , req : Req) -> Self :: Future { Box :: pin (self . inner . call (req)) } }
};
}
