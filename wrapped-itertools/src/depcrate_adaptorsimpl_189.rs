// Generated macro for impl_189 (impl)
macro_rules! Depcrate_adaptorsimpl_189 {
() => {
// Module: crate::adaptors
// Provides: {"impl_189"}
// Dependencies: {}
impl < I , F > DoubleEndedIterator for Update < I , F > where I : DoubleEndedIterator , F : FnMut (& mut I :: Item) , { fn next_back (& mut self) -> Option < Self :: Item > { if let Some (mut v) = self . iter . next_back () { (self . f) (& mut v) ; Some (v) } else { None } } }
};
}
