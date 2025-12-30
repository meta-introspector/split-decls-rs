// Generated macro for impl_29 (impl)
macro_rules! Depcrate_oomimpl_29 {
() => {
// Module: crate::oom
// Provides: {"impl_29"}
// Dependencies: {}
impl R { fn new (cnt : & 'static AtomicUsize , panic_free_drop : bool) -> R { let boxed = Box :: new ((cnt , panic_free_drop)) ; cnt . fetch_add (1 , AcqRel) ; R (boxed) } }
};
}
