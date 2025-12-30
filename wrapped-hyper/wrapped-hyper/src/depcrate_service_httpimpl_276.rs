// Generated macro for impl_276 (impl)
macro_rules! Depcrate_service_httpimpl_276 {
() => {
// Module: crate::service::http
// Provides: {"impl_276"}
// Dependencies: {}
impl < T , B1 , B2 > HttpService < B1 > for T where T : Service < Request < B1 > , Response = Response < B2 > > , B2 : Body , T :: Error : Into < Box < dyn StdError + Send + Sync > > , { type ResBody = B2 ; type Error = T :: Error ; type Future = T :: Future ; fn call (& mut self , req : Request < B1 >) -> Self :: Future { Service :: call (self , req) } }
};
}
