// Generated macro for impl_256 (impl)
macro_rules! Depcrateimpl_256 {
() => {
// Module: crate
// Provides: {"impl_256"}
// Dependencies: {}
impl < 'a , S , Req > Service < Req > for & 'a S where S : Service < Req > + 'a , { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; fn poll_ready (& self , ctx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { (* * self) . poll_ready (ctx) } fn call (& self , request : Req) -> S :: Future { (* * self) . call (request) } }
};
}
