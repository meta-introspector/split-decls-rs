// Generated macro for impl_285 (impl)
macro_rules! Depcrate_service_serviceimpl_285 {
() => {
// Module: crate::service::service
// Provides: {"impl_285"}
// Dependencies: {}
impl < Request , S : Service < Request > + ? Sized > Service < Request > for std :: rc :: Rc < S > { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; # [inline] fn call (& self , req : Request) -> Self :: Future { (* * self) . call (req) } }
};
}
