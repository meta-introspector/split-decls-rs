// Generated macro for impl_209 (impl)
macro_rules! Depcrate_selectimpl_209 {
() => {
// Module: crate::select
// Provides: {"impl_209"}
// Dependencies: {}
impl < A , B > Future for SelectNext < A , B > where A : Future , B : Future < Item = A :: Item , Error = A :: Error > , { type Item = A :: Item ; type Error = A :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < Self :: Item , Self :: Error > { match self . inner { OneOf :: A (ref mut a) => a . poll (task) , OneOf :: B (ref mut b) => b . poll (task) , } } fn schedule (& mut self , task : & mut Task) { match self . inner { OneOf :: A (ref mut a) => a . schedule (task) , OneOf :: B (ref mut b) => b . schedule (task) , } } fn tailcall (& mut self) -> Option < Box < Future < Item = Self :: Item , Error = Self :: Error > > > { match self . inner { OneOf :: A (ref mut a) => a . collapse () , OneOf :: B (ref mut b) => b . collapse () , } match self . inner { OneOf :: A (Collapsed :: Tail (ref mut a)) | OneOf :: B (Collapsed :: Tail (ref mut a)) => { Some (mem :: replace (a , Box :: new (empty ()))) } _ => None , } } }
};
}
