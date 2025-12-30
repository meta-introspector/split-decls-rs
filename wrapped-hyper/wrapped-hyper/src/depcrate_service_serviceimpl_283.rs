// Generated macro for impl_283 (impl)
macro_rules! Depcrate_service_serviceimpl_283 {
() => {
// Module: crate::service::service
// Provides: {"impl_283"}
// Dependencies: {}
impl < Request , S : Service < Request > + ? Sized > Service < Request > for & '_ mut S { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; # [inline] fn call (& self , req : Request) -> Self :: Future { (* * self) . call (req) } }
};
}
