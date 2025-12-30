// Generated macro for impl_417 (impl)
macro_rules! Depcrate_forgetimpl_417 {
() => {
// Module: crate::forget
// Provides: {"impl_417"}
// Dependencies: {}
impl < T : Send + 'static , E : Send + 'static > Future for ThunkFuture < T , E > { type Item = () ; type Error = () ; fn poll (& mut self , task : & mut Task) -> Poll < () , () > { self . inner . poll (task) . map (| _ | ()) . map_err (| _ | ()) } fn schedule (& mut self , task : & mut Task) { self . inner . schedule (task) } fn tailcall (& mut self) -> Option < Box < Future < Item = () , Error = () > > > { if let Some (f) = self . inner . tailcall () { self . inner = f ; } None } }
};
}
