// Generated macro for impl_508 (impl)
macro_rules! Depcrate_future_eitherimpl_508 {
() => {
// Module: crate::future::either
// Provides: {"impl_508"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < A , B , Item > Sink < Item > for Either < A , B > where A : Sink < Item > , B : Sink < Item , Error = A :: Error > , { type Error = A :: Error ; fn poll_ready (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { match self . as_pin_mut () { Either :: Left (x) => x . poll_ready (cx) , Either :: Right (x) => x . poll_ready (cx) , } } fn start_send (self : Pin < & mut Self > , item : Item) -> Result < () , Self :: Error > { match self . as_pin_mut () { Either :: Left (x) => x . start_send (item) , Either :: Right (x) => x . start_send (item) , } } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { match self . as_pin_mut () { Either :: Left (x) => x . poll_flush (cx) , Either :: Right (x) => x . poll_flush (cx) , } } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { match self . as_pin_mut () { Either :: Left (x) => x . poll_close (cx) , Either :: Right (x) => x . poll_close (cx) , } } }
};
}
