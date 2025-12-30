// Generated macro for impl_13 (impl)
macro_rules! Depcrate_and_thenimpl_13 {
() => {
// Module: crate::and_then
// Provides: {"impl_13"}
// Dependencies: {}
impl < A , B , Req > Service < Req > for AndThenService < A , B , Req > where A : Service < Req > , B : Service < A :: Response , Error = A :: Error > , { type Response = B :: Response ; type Error = A :: Error ; type Future = AndThenServiceResponse < A , B , Req > ; fn poll_ready (& self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { let (a , b) = & * self . 0 ; let not_ready = ! a . poll_ready (cx) ? . is_ready () ; if ! b . poll_ready (cx) ? . is_ready () || not_ready { Poll :: Pending } else { Poll :: Ready (Ok (())) } } fn call (& self , req : Req) -> Self :: Future { AndThenServiceResponse { state : State :: A { fut : self . 0 . 0 . call (req) , b : Some (self . 0 . clone ()) , } , } } }
};
}
