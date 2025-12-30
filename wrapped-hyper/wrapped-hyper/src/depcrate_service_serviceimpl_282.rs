// Generated macro for impl_282 (impl)
macro_rules! Depcrate_service_serviceimpl_282 {
() => {
// Module: crate::service::service
// Provides: {"impl_282"}
// Dependencies: {}
impl < Request , S : Service < Request > + ? Sized > Service < Request > for & '_ S { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; # [inline] fn call (& self , req : Request) -> Self :: Future { (* * self) . call (req) } }
};
}
