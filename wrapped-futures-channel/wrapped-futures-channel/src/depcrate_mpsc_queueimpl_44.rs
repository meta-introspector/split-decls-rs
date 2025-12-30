// Generated macro for impl_44 (impl)
macro_rules! Depcrate_mpsc_queueimpl_44 {
() => {
// Module: crate::mpsc::queue
// Provides: {"impl_44"}
// Dependencies: {}
impl < T > Drop for Queue < T > { fn drop (& mut self) { unsafe { let mut cur = * self . tail . get () ; while ! cur . is_null () { let next = (* cur) . next . load (Ordering :: Relaxed) ; drop (Box :: from_raw (cur)) ; cur = next ; } } } }
};
}
