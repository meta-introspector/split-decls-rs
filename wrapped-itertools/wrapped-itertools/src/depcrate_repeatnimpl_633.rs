// Generated macro for impl_633 (impl)
macro_rules! Depcrate_repeatnimpl_633 {
() => {
// Module: crate::repeatn
// Provides: {"impl_633"}
// Dependencies: {}
impl < A > DoubleEndedIterator for RepeatN < A > where A : Clone , { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { self . next () } # [inline] fn rfold < B , F > (self , init : B , f : F) -> B where F : FnMut (B , Self :: Item) -> B , { self . fold (init , f) } }
};
}
