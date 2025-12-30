// Generated macro for impl_257 (impl)
macro_rules! Depcrateimpl_257 {
() => {
// Module: crate
// Provides: {"impl_257"}
// Dependencies: {}
impl < S , Req > Service < Req > for Box < S > where S : Service < Req > + ? Sized , { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; fn poll_ready (& self , ctx : & mut Context < '_ >) -> Poll < Result < () , S :: Error > > { (* * self) . poll_ready (ctx) } fn call (& self , request : Req) -> S :: Future { (* * self) . call (request) } }
};
}
