// Generated macro for impl_551 (impl)
macro_rules! Depcrate_spscimpl_551 {
() => {
// Module: crate::spsc
// Provides: {"impl_551"}
// Dependencies: {}
impl < 'a , T > Iterator for Iter < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { if self . index < self . len { let head = self . rb . head . load (Ordering :: Relaxed) ; let i = (head + self . index) % self . rb . n () ; self . index += 1 ; Some (unsafe { & * (self . rb . buffer . borrow () . get_unchecked (i) . get () as * const T) }) } else { None } } }
};
}
