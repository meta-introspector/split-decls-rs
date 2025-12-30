// Generated macro for impl_24 (impl)
macro_rules! Depcrate_dequeimpl_24 {
() => {
// Module: crate::deque
// Provides: {"impl_24"}
// Dependencies: {}
impl < T > Drop for Inner < T > { fn drop (& mut self) { let b = * self . back . get_mut () ; let f = * self . front . get_mut () ; unsafe { let buffer = self . buffer . load (Ordering :: Relaxed , epoch :: unprotected ()) ; let mut i = f ; while i != b { buffer . deref () . at (i) . drop_in_place () ; i = i . wrapping_add (1) ; } buffer . into_owned () . into_box () . dealloc () ; } } }
};
}
