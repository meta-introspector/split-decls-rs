// Generated macro for impl_24 (impl)
macro_rules! Depcrate_boxedimpl_24 {
() => {
// Module: crate::boxed
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'a , I : Iterator + ? Sized > Iterator for Box < 'a , I > { type Item = I :: Item ; fn next (& mut self) -> Option < I :: Item > { (* * self) . next () } fn size_hint (& self) -> (usize , Option < usize >) { (* * self) . size_hint () } fn nth (& mut self , n : usize) -> Option < I :: Item > { (* * self) . nth (n) } fn last (self) -> Option < I :: Item > { # [inline] fn some < T > (_ : Option < T > , x : T) -> Option < T > { Some (x) } self . fold (None , some) } }
};
}
