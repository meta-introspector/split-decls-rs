// Generated macro for impl_253 (impl)
macro_rules! Depcrateimpl_253 {
() => {
// Module: crate
// Provides: {"impl_253"}
// Dependencies: {}
impl Drop for Execs { fn drop (& mut self) { if ! self . ran && ! std :: thread :: panicking () { panic ! ("forgot to run this command") ; } } }
};
}
