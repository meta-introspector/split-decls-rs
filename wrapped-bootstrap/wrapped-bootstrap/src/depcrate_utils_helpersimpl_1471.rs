// Generated macro for impl_1471 (impl)
macro_rules! Depcrate_utils_helpersimpl_1471 {
() => {
// Module: crate::utils::helpers
// Provides: {"impl_1471"}
// Dependencies: {}
impl Drop for PanicTracker < '_ > { fn drop (& mut self) { if panicking () { eprintln ! ("Panic was initiated from {}:{}:{}" , self . 0 . file () , self . 0 . line () , self . 0 . column ()) ; } } }
};
}
