// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
# [cfg (feature = "std")] impl < I : ? Sized > FallibleStreamingIterator for Box < I > where I : FallibleStreamingIterator , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn advance (& mut self) -> Result < () , I :: Error > { (* * self) . advance () } # [inline] fn get (& self) -> Option < & I :: Item > { (* * self) . get () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (* * self) . size_hint () } # [inline] fn next (& mut self) -> Result < Option < & I :: Item > , I :: Error > { (* * self) . next () } }
};
}
