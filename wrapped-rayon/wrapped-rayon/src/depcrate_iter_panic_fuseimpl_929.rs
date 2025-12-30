// Generated macro for impl_929 (impl)
macro_rules! Depcrate_iter_panic_fuseimpl_929 {
() => {
// Module: crate::iter::panic_fuse
// Provides: {"impl_929"}
// Dependencies: {}
impl < 'a > Drop for Fuse < 'a > { # [inline] fn drop (& mut self) { if thread :: panicking () { self . 0 . store (true , Ordering :: Relaxed) ; } } }
};
}
