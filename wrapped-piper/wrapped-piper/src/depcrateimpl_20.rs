// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl Drop for Reader { fn drop (& mut self) { self . inner . closed . store (true , Ordering :: SeqCst) ; self . inner . writer . wake () ; } }
};
}
