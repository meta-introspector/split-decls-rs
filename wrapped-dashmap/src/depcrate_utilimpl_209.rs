// Generated macro for impl_209 (impl)
macro_rules! Depcrate_utilimpl_209 {
() => {
// Module: crate::util
// Provides: {"impl_209"}
// Dependencies: {}
impl Drop for AbortOnPanic { fn drop (& mut self) { if std :: thread :: panicking () { std :: process :: abort () } } }
};
}
