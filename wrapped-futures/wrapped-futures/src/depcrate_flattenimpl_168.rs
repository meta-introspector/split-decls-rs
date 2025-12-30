// Generated macro for impl_168 (impl)
macro_rules! Depcrate_flattenimpl_168 {
() => {
// Module: crate::flatten
// Provides: {"impl_168"}
// Dependencies: {}
impl < A > Future for Flatten < A > where A : Future , A :: Item : IntoFuture , < < A as Future > :: Item as IntoFuture > :: Error : From < < A as Future > :: Error > { type Item = < < A as Future > :: Item as IntoFuture > :: Item ; type Error = < < A as Future > :: Item as IntoFuture > :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < Self :: Item , Self :: Error > { self . state . poll (task , | a , () | { let future = try ! (a) . into_future () ; Ok (Err (future)) }) } fn schedule (& mut self , task : & mut Task) { self . state . schedule (task) } fn tailcall (& mut self) -> Option < Box < Future < Item = Self :: Item , Error = Self :: Error > > > { self . state . tailcall () } }
};
}
