// Generated macro for impl_412 (impl)
macro_rules! Depcrate_implsimpl_412 {
() => {
// Module: crate::impls
// Provides: {"impl_412"}
// Dependencies: {}
impl < F : Future > Future for Box < F > { type Item = F :: Item ; type Error = F :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < Self :: Item , Self :: Error > { (* * self) . poll (task) } fn schedule (& mut self , task : & mut Task) { (* * self) . schedule (task) } fn tailcall (& mut self) -> Option < Box < Future < Item = Self :: Item , Error = Self :: Error > > > { (* * self) . tailcall () } }
};
}
