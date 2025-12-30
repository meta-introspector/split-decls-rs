// Generated macro for impl_33 (impl)
macro_rules! Depcrate_streamimpl_33 {
() => {
// Module: crate::stream
// Provides: {"impl_33"}
// Dependencies: {}
impl Stream for JsStream { type Item = Result < JsValue , JsValue > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context) -> Poll < Option < Self :: Item > > { if self . done { return Poll :: Ready (None) ; } let future = match self . next . as_mut () { Some (val) => val , None => match self . next_future () { Ok (val) => { self . next = Some (val) ; self . next . as_mut () . unwrap () } Err (e) => { self . done = true ; return Poll :: Ready (Some (Err (e))) ; } } , } ; match Pin :: new (future) . poll (cx) { Poll :: Ready (res) => match res { Ok (iter_next) => { let next = iter_next . unchecked_into :: < IteratorNext > () ; if next . done () { self . done = true ; Poll :: Ready (None) } else { self . next . take () ; Poll :: Ready (Some (Ok (next . value ()))) } } Err (e) => { self . done = true ; Poll :: Ready (Some (Err (e))) } } , Poll :: Pending => Poll :: Pending , } } }
};
}
