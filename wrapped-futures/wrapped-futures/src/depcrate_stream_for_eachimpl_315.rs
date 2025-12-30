// Generated macro for impl_315 (impl)
macro_rules! Depcrate_stream_for_eachimpl_315 {
() => {
// Module: crate::stream::for_each
// Provides: {"impl_315"}
// Dependencies: {}
impl < S , F > Future for ForEach < S , F > where S : Stream , F : FnMut (S :: Item) -> Result < () , S :: Error > + Send + 'static { type Item = () ; type Error = S :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < () , S :: Error > { loop { match try_poll ! (self . stream . poll (task)) { Ok (Some (e)) => { match (self . f) (e) { Ok (()) => { } Err (e) => return Poll :: Err (e) , } } Ok (None) => return Poll :: Ok (()) , Err (e) => return Poll :: Err (e) , } } } fn schedule (& mut self , task : & mut Task) { self . stream . schedule (task) } }
};
}
