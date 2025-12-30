// Generated macro for impl_309 (impl)
macro_rules! Depcrate_stream_foldimpl_309 {
() => {
// Module: crate::stream::fold
// Provides: {"impl_309"}
// Dependencies: {}
impl < S , F , Fut , T > Future for Fold < S , F , Fut , T > where S : Stream , F : FnMut (T , S :: Item) -> Fut + Send + 'static , Fut : IntoFuture < Item = T > , Fut :: Error : Into < S :: Error > , T : Send + 'static { type Item = T ; type Error = S :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < T , S :: Error > { loop { match mem :: replace (& mut self . state , State :: Empty) { State :: Empty => panic ! ("cannot poll Fold twice") , State :: Ready (state) => { match self . stream . poll (task) { Poll :: Ok (Some (e)) => { let future = (self . f) (state , e) ; self . state = State :: Processing (future . into_future ()) ; } Poll :: Ok (None) => return Poll :: Ok (state) , Poll :: Err (e) => return Poll :: Err (e) , Poll :: NotReady => { self . state = State :: Ready (state) ; return Poll :: NotReady } } } State :: Processing (mut fut) => { match fut . poll (task) { Poll :: Ok (state) => self . state = State :: Ready (state) , Poll :: Err (e) => return Poll :: Err (e . into ()) , Poll :: NotReady => { self . state = State :: Processing (fut) ; return Poll :: NotReady ; } } } } } } fn schedule (& mut self , task : & mut Task) { match self . state { State :: Empty => panic ! ("cannot `schedule` a completed Fold") , State :: Ready (_) => self . stream . schedule (task) , State :: Processing (ref mut fut) => fut . schedule (task) , } } }
};
}
