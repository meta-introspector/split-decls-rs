// Generated macro for impl_217 (impl)
macro_rules! Depcrate_select_allimpl_217 {
() => {
// Module: crate::select_all
// Provides: {"impl_217"}
// Dependencies: {}
impl < A > Future for SelectAll < A > where A : Future , { type Item = (A :: Item , usize , Vec < SelectAllNext < A > >) ; type Error = (A :: Error , usize , Vec < SelectAllNext < A > >) ; fn poll (& mut self , task : & mut Task) -> Poll < Self :: Item , Self :: Error > { let item = self . inner . iter_mut () . enumerate () . filter_map (| (i , f) | { match f . poll (task) { Poll :: NotReady => None , Poll :: Ok (e) => Some ((i , Ok (e))) , Poll :: Err (e) => Some ((i , Err (e))) , } }) . next () ; match item { Some ((idx , res)) => { self . inner . remove (idx) ; let rest = mem :: replace (& mut self . inner , Vec :: new ()) ; match res { Ok (e) => Poll :: Ok ((e , idx , rest)) , Err (e) => Poll :: Err ((e , idx , rest)) , } } None => Poll :: NotReady , } } fn schedule (& mut self , task : & mut Task) { for f in self . inner . iter_mut () { f . inner . schedule (task) ; } } fn tailcall (& mut self) -> Option < Box < Future < Item = Self :: Item , Error = Self :: Error > > > { for f in self . inner . iter_mut () { f . inner . collapse () ; } None } }
};
}
