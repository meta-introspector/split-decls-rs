// Generated macro for impl_100 (impl)
macro_rules! Depcrate_address_queueimpl_100 {
() => {
// Module: crate::address::queue
// Provides: {"impl_100"}
// Dependencies: {}
impl < T > Drop for Queue < T > { fn drop (& mut self) { unsafe { let mut cur = * self . tail . get () ; while ! cur . is_null () { let next = (* cur) . next . load (Ordering :: Relaxed) ; drop (Box :: from_raw (cur)) ; cur = next ; } } } }
};
}
