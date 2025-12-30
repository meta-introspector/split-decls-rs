// Generated macro for impl_59 (impl)
macro_rules! Depcrateimpl_59 {
() => {
// Module: crate
// Provides: {"impl_59"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < I : FallibleIterator + ? Sized > FallibleIterator for Box < I > { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn next (& mut self) -> Result < Option < I :: Item > , I :: Error > { (* * self) . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (* * self) . size_hint () } # [inline] fn nth (& mut self , n : usize) -> Result < Option < I :: Item > , I :: Error > { (* * self) . nth (n) } }
};
}
