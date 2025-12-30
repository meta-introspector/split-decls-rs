// Generated macro for impl_553 (impl)
macro_rules! Depcrate_spscimpl_553 {
() => {
// Module: crate::spsc
// Provides: {"impl_553"}
// Dependencies: {}
impl < T > DoubleEndedIterator for Iter < '_ , T > { fn next_back (& mut self) -> Option < Self :: Item > { if self . index < self . len { let head = self . rb . head . load (Ordering :: Relaxed) ; let i = (head + self . len - 1) % self . rb . n () ; self . len -= 1 ; Some (unsafe { & * (self . rb . buffer . borrow () . get_unchecked (i) . get () as * const T) }) } else { None } } }
};
}
