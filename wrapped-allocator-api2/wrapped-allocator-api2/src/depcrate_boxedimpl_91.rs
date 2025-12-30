// Generated macro for impl_91 (impl)
macro_rules! Depcrate_boxedimpl_91 {
() => {
// Module: crate::boxed
// Provides: {"impl_91"}
// Dependencies: {}
impl < I : Iterator + ? Sized , A : Allocator > Iterator for Box < I , A > { type Item = I :: Item ; # [inline (always)] fn next (& mut self) -> Option < I :: Item > { (* * self) . next () } # [inline (always)] fn size_hint (& self) -> (usize , Option < usize >) { (* * self) . size_hint () } # [inline (always)] fn nth (& mut self , n : usize) -> Option < I :: Item > { (* * self) . nth (n) } # [inline (always)] fn last (self) -> Option < I :: Item > { BoxIter :: last (self) } }
};
}
