// Generated macro for impl_61 (impl)
macro_rules! Depcrateimpl_61 {
() => {
// Module: crate
// Provides: {"impl_61"}
// Dependencies: {}
impl < B : BitBlock > DoubleEndedIterator for IterMut < '_ , B > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { let index = self . range . next_back () ; self . get (index) } }
};
}
