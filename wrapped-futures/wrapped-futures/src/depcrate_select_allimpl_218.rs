// Generated macro for impl_218 (impl)
macro_rules! Depcrate_select_allimpl_218 {
() => {
// Module: crate::select_all
// Provides: {"impl_218"}
// Dependencies: {}
impl < A > Future for SelectAllNext < A > where A : Future , { type Item = A :: Item ; type Error = A :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < Self :: Item , Self :: Error > { self . inner . poll (task) } fn schedule (& mut self , task : & mut Task) { self . inner . schedule (task) } fn tailcall (& mut self) -> Option < Box < Future < Item = Self :: Item , Error = Self :: Error > > > { self . inner . collapse () ; match self . inner { Collapsed :: Tail (ref mut a) => { Some (mem :: replace (a , Box :: new (empty ()))) } _ => None , } } }
};
}
