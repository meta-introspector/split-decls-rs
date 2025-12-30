// Generated macro for impl_275 (impl)
macro_rules! Depcrate_stream_bufferedimpl_275 {
() => {
// Module: crate::stream::buffered
// Provides: {"impl_275"}
// Dependencies: {}
impl < S > Stream for Buffered < S > where S : Stream , S :: Item : IntoFuture < Error = < S as Stream > :: Error > , { type Item = < S :: Item as IntoFuture > :: Item ; type Error = < S as Stream > :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < Option < Self :: Item > , Self :: Error > { let mut any_some = false ; for f in self . futures . iter_mut () { if f . is_none () { match self . stream . poll (task) { Poll :: Ok (Some (e)) => { * f = Some (Collapsed :: Start (e . into_future ())) ; } Poll :: Err (e) => return Poll :: Err (e) , Poll :: Ok (None) | Poll :: NotReady => continue , } } let ret = { let future = f . as_mut () . unwrap () ; match future . poll (task) { Poll :: Ok (e) => Poll :: Ok (Some (e)) , Poll :: Err (e) => Poll :: Err (e) , Poll :: NotReady => { future . collapse () ; any_some = true ; continue } } } ; * f = None ; return ret } if any_some || ! self . stream . is_done () { Poll :: NotReady } else { Poll :: Ok (None) } } fn schedule (& mut self , task : & mut Task) { let mut any_none = false ; for f in self . futures . iter_mut () { match * f { Some (ref mut f) => f . schedule (task) , None => any_none = true , } } if any_none { self . stream . schedule (task) ; } } }
};
}
