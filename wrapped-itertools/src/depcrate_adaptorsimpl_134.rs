// Generated macro for impl_134 (impl)
macro_rules! Depcrate_adaptorsimpl_134 {
() => {
// Module: crate::adaptors
// Provides: {"impl_134"}
// Dependencies: {}
impl < B , F , I > Iterator for Batching < I , F > where I : Iterator , F : FnMut (& mut I) -> Option < B > , { type Item = B ; # [inline] fn next (& mut self) -> Option < Self :: Item > { (self . f) (& mut self . iter) } }
};
}
