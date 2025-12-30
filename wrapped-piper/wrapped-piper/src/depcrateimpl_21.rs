// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl Drop for Writer { fn drop (& mut self) { self . inner . closed . store (true , Ordering :: SeqCst) ; self . inner . reader . wake () ; } }
};
}
