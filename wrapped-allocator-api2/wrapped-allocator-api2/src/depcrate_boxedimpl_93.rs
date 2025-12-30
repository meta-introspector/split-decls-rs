// Generated macro for impl_93 (impl)
macro_rules! Depcrate_boxedimpl_93 {
() => {
// Module: crate::boxed
// Provides: {"impl_93"}
// Dependencies: {}
impl < I : Iterator + ? Sized , A : Allocator > BoxIter for Box < I , A > { type Item = I :: Item ; # [inline (always)] fn last (self) -> Option < I :: Item > { # [inline (always)] fn some < T > (_ : Option < T > , x : T) -> Option < T > { Some (x) } self . fold (None , some) } }
};
}
