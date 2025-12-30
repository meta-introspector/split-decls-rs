// Generated macro for impl_272 (impl)
macro_rules! Depcrate_scopeimpl_272 {
() => {
// Module: crate::scope
// Provides: {"impl_272"}
// Dependencies: {}
impl < 'scope > fmt :: Debug for Scope < 'scope > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("Scope") . field ("pool_id" , & self . base . registry . id ()) . field ("panic" , & self . base . panic) . field ("job_completed_latch" , & self . base . job_completed_latch) . finish () } }
};
}
