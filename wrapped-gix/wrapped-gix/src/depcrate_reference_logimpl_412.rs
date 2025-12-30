// Generated macro for impl_412 (impl)
macro_rules! Depcrate_reference_logimpl_412 {
() => {
// Module: crate::reference::log
// Provides: {"impl_412"}
// Dependencies: {}
impl Reference < '_ > { # [doc = " Return a platform for obtaining iterators over reference logs."] pub fn log_iter (& self) -> gix_ref :: file :: log :: iter :: Platform < '_ , '_ > { self . inner . log_iter (& self . repo . refs) } # [doc = " Return true if a reflog is present for this reference."] pub fn log_exists (& self) -> bool { self . inner . log_exists (& self . repo . refs) } }
};
}
