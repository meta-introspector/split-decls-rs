// Generated macro for impl_693 (impl)
macro_rules! Depcrate_stream_stream_anyimpl_693 {
() => {
// Module: crate::stream::stream::any
// Provides: {"impl_693"}
// Dependencies: {}
impl < St , Fut , F > Future for Any < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = bool > , { type Output = bool ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < bool > { let mut this = self . project () ; Poll :: Ready (loop { if let Some (fut) = this . future . as_mut () . as_pin_mut () { let res = ready ! (fut . poll (cx)) ; this . future . set (None) ; if res { * this . done = true ; break true ; } } else if ! * this . done { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (item) => { this . future . set (Some ((this . f) (item))) ; } None => { * this . done = true ; break false ; } } } else { panic ! ("Any polled after completion") } }) } }
};
}
