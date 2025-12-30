// Generated macro for impl_77 (impl)
macro_rules! Depcrate_jobimpl_77 {
() => {
// Module: crate::job
// Provides: {"impl_77"}
// Dependencies: {}
impl Job for JobFifo { unsafe fn execute (this : * const ()) { unsafe { let this = & * (this as * const Self) ; loop { match this . inner . steal () { Steal :: Success (job_ref) => break job_ref . execute () , Steal :: Empty => panic ! ("FIFO is empty") , Steal :: Retry => { } } } } } }
};
}
