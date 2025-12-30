// Generated macro for impl_89 (impl)
macro_rules! Depcrate_arrayvecimpl_89 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_89"}
// Dependencies: {}
impl < T , const CAP : usize > Iterator for IntoIter < T , CAP > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { if self . index == self . v . len () { None } else { unsafe { let index = self . index ; self . index = index + 1 ; Some (ptr :: read (self . v . get_unchecked_ptr (index))) } } } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . v . len () - self . index ; (len , Some (len)) } }
};
}
