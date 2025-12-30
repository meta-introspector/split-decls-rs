// Generated macro for impl_173 (impl)
macro_rules! Depcrate_fuseimpl_173 {
() => {
// Module: crate::fuse
// Provides: {"impl_173"}
// Dependencies: {}
impl < A : Future > Future for Fuse < A > { type Item = A :: Item ; type Error = A :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < A :: Item , A :: Error > { let ret = self . future . as_mut () . map (| f | f . poll (task)) ; if ret . as_ref () . map (| r | r . is_ready ()) == Some (true) { self . future = None ; } return ret . unwrap_or (Poll :: NotReady) } fn schedule (& mut self , task : & mut Task) { if let Some (ref mut f) = self . future { f . schedule (task) ; } } }
};
}
