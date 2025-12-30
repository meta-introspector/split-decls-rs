// Generated macro for impl_284 (impl)
macro_rules! Depcrate_service_serviceimpl_284 {
() => {
// Module: crate::service::service
// Provides: {"impl_284"}
// Dependencies: {}
impl < Request , S : Service < Request > + ? Sized > Service < Request > for Box < S > { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; # [inline] fn call (& self , req : Request) -> Self :: Future { (* * self) . call (req) } }
};
}
