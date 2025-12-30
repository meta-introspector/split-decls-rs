// Generated macro for impl_260 (impl)
macro_rules! Depcrate_stream_iterimpl_260 {
() => {
// Module: crate::stream::iter
// Provides: {"impl_260"}
// Dependencies: {}
impl < I , T , E > Stream for IterStream < I > where I : Iterator < Item = Result < T , E > > , I : Send + 'static , T : Send + 'static , E : Send + 'static , { type Item = T ; type Error = E ; fn poll (& mut self , _task : & mut Task) -> Poll < Option < T > , E > { match self . iter . next () { Some (Ok (e)) => Poll :: Ok (Some (e)) , Some (Err (e)) => Poll :: Err (e) , None => Poll :: Ok (None) , } } fn schedule (& mut self , task : & mut Task) { task . notify () } }
};
}
