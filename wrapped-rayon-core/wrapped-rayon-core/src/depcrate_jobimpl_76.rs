// Generated macro for impl_76 (impl)
macro_rules! Depcrate_jobimpl_76 {
() => {
// Module: crate::job
// Provides: {"impl_76"}
// Dependencies: {}
impl JobFifo { pub (super) fn new () -> Self { JobFifo { inner : Injector :: new () , } } pub (super) unsafe fn push (& self , job_ref : JobRef) -> JobRef { unsafe { self . inner . push (job_ref) ; JobRef :: new (self) } } }
};
}
