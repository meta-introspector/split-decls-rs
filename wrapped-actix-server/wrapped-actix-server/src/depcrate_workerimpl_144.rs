// Generated macro for impl_144 (impl)
macro_rules! Depcrate_workerimpl_144 {
() => {
// Module: crate::worker
// Provides: {"impl_144"}
// Dependencies: {}
impl Counter { pub (crate) fn new (limit : usize) -> Self { Self { counter : Arc :: new (AtomicUsize :: new (1)) , limit , } } # [doc = " Increment counter by 1 and return true when hitting limit"] # [inline (always)] pub (crate) fn inc (& self) -> bool { self . counter . fetch_add (1 , Ordering :: Relaxed) != self . limit } # [doc = " Decrement counter by 1 and return true if crossing limit."] # [inline (always)] pub (crate) fn dec (& self) -> bool { self . counter . fetch_sub (1 , Ordering :: Relaxed) == self . limit } pub (crate) fn total (& self) -> usize { self . counter . load (Ordering :: SeqCst) - 1 } }
};
}
