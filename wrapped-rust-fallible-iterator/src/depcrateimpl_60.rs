// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < I : DoubleEndedFallibleIterator + ? Sized > DoubleEndedFallibleIterator for Box < I > { # [inline] fn next_back (& mut self) -> Result < Option < I :: Item > , I :: Error > { (* * self) . next_back () } }
};
}
