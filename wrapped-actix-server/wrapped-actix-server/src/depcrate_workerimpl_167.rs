// Generated macro for impl_167 (impl)
macro_rules! Depcrate_workerimpl_167 {
() => {
// Module: crate::worker
// Provides: {"impl_167"}
// Dependencies: {}
impl Drop for ServerWorker { fn drop (& mut self) { Arbiter :: try_current () . as_ref () . map (ArbiterHandle :: stop) ; } }
};
}
