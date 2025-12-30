// Generated macro for impl_365 (impl)
macro_rules! Depcrate_stream_skip_whileimpl_365 {
() => {
// Module: crate::stream::skip_while
// Provides: {"impl_365"}
// Dependencies: {}
impl < S , P , R > Stream for SkipWhile < S , P , R > where S : Stream , P : FnMut (& S :: Item) -> R + Send + 'static , R : IntoFuture < Item = bool , Error = S :: Error > , { type Item = S :: Item ; type Error = S :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < Option < S :: Item > , S :: Error > { if self . done_skipping { return self . stream . poll (task) ; } loop { if self . pending . is_none () { let item = match try_poll ! (self . stream . poll (task)) { Ok (Some (e)) => e , Ok (None) => return Poll :: Ok (None) , Err (e) => return Poll :: Err (e) , } ; self . pending = Some (((self . pred) (& item) . into_future () , item)) ; } assert ! (self . pending . is_some ()) ; match try_poll ! (self . pending . as_mut () . unwrap () . 0 . poll (task)) { Ok (true) => self . pending = None , Ok (false) => { let (_ , item) = self . pending . take () . unwrap () ; self . done_skipping = true ; return Poll :: Ok (Some (item)) } Err (e) => { self . pending = None ; return Poll :: Err (e) } } } } fn schedule (& mut self , task : & mut Task) { self . stream . schedule (task) } }
};
}
