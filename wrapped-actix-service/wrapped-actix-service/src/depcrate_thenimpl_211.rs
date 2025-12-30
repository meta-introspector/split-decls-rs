// Generated macro for impl_211 (impl)
macro_rules! Depcrate_thenimpl_211 {
() => {
// Module: crate::then
// Provides: {"impl_211"}
// Dependencies: {}
impl < A , B , Req > Service < Req > for ThenService < A , B , Req > where A : Service < Req > , B : Service < Result < A :: Response , A :: Error > , Error = A :: Error > , { type Response = B :: Response ; type Error = B :: Error ; type Future = ThenServiceResponse < A , B , Req > ; fn poll_ready (& self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { let (a , b) = & * self . 0 ; let not_ready = ! a . poll_ready (cx) ? . is_ready () ; if ! b . poll_ready (cx) ? . is_ready () || not_ready { Poll :: Pending } else { Poll :: Ready (Ok (())) } } fn call (& self , req : Req) -> Self :: Future { ThenServiceResponse { state : State :: A { fut : self . 0 . 0 . call (req) , b : Some (self . 0 . clone ()) , } , } } }
};
}
