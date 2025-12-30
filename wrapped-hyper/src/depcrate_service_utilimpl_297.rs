// Generated macro for impl_297 (impl)
macro_rules! Depcrate_service_utilimpl_297 {
() => {
// Module: crate::service::util
// Provides: {"impl_297"}
// Dependencies: {}
impl < F , ReqBody , Ret , ResBody , E > Service < Request < ReqBody > > for ServiceFn < F , ReqBody > where F : Fn (Request < ReqBody >) -> Ret , ReqBody : Body , Ret : Future < Output = Result < Response < ResBody > , E > > , E : Into < Box < dyn StdError + Send + Sync > > , ResBody : Body , { type Response = crate :: Response < ResBody > ; type Error = E ; type Future = Ret ; fn call (& self , req : Request < ReqBody >) -> Self :: Future { (self . f) (req) } }
};
}
