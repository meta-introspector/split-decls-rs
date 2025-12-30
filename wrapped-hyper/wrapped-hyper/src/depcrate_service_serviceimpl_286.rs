// Generated macro for impl_286 (impl)
macro_rules! Depcrate_service_serviceimpl_286 {
() => {
// Module: crate::service::service
// Provides: {"impl_286"}
// Dependencies: {}
impl < Request , S : Service < Request > + ? Sized > Service < Request > for std :: sync :: Arc < S > { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; # [inline] fn call (& self , req : Request) -> Self :: Future { (* * self) . call (req) } }
};
}
