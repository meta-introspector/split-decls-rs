// Generated macro for impl_552 (impl)
macro_rules! Depcrate_spscimpl_552 {
() => {
// Module: crate::spsc
// Provides: {"impl_552"}
// Dependencies: {}
impl < 'a , T > Iterator for IterMut < 'a , T > { type Item = & 'a mut T ; fn next (& mut self) -> Option < Self :: Item > { if self . index < self . len { let head = self . rb . head . load (Ordering :: Relaxed) ; let i = (head + self . index) % self . rb . n () ; self . index += 1 ; Some (unsafe { & mut * self . rb . buffer . borrow () . get_unchecked (i) . get () . cast :: < T > () }) } else { None } } }
};
}
