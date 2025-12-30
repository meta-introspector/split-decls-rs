// Generated macro for impl_57 (impl)
macro_rules! Depcrateimpl_57 {
() => {
// Module: crate
// Provides: {"impl_57"}
// Dependencies: {}
impl < I : FallibleIterator + ? Sized > FallibleIterator for & mut I { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn next (& mut self) -> Result < Option < I :: Item > , I :: Error > { (* * self) . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (* * self) . size_hint () } # [inline] fn nth (& mut self , n : usize) -> Result < Option < I :: Item > , I :: Error > { (* * self) . nth (n) } }
};
}
