// Generated macro for impl_96 (impl)
macro_rules! Depcrate_collectimpl_96 {
() => {
// Module: crate::collect
// Provides: {"impl_96"}
// Dependencies: {}
impl < I > Future for Collect < I > where I : IntoIterator + Send + 'static , I :: IntoIter : Send + 'static , I :: Item : IntoFuture , { type Item = Vec < < I :: Item as IntoFuture > :: Item > ; type Error = < I :: Item as IntoFuture > :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < Self :: Item , Self :: Error > { loop { match self . cur { Some (ref mut cur) => { match try_poll ! (cur . poll (task)) { Ok (e) => self . result . push (e) , Err (e) => { for f in self . remaining . by_ref () { drop (f) ; } for f in self . result . drain (..) { drop (f) ; } return Poll :: Err (e) } } } None => { return Poll :: Ok (mem :: replace (& mut self . result , Vec :: new ())) } } self . cur = self . remaining . next () . map (IntoFuture :: into_future) . map (Collapsed :: Start) ; } } fn schedule (& mut self , task : & mut Task) { if let Some (ref mut cur) = self . cur { cur . schedule (task) ; } } fn tailcall (& mut self) -> Option < Box < Future < Item = Self :: Item , Error = Self :: Error > > > { if let Some (ref mut cur) = self . cur { cur . collapse () ; } None } }
};
}
