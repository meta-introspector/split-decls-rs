// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl < I , F , B : ? Sized > DoubleEndedFallibleStreamingIterator for MapRef < I , F > where I : DoubleEndedFallibleStreamingIterator , F : Fn (& I :: Item) -> & B , { # [inline] fn advance_back (& mut self) -> Result < () , I :: Error > { self . it . advance_back () } }
};
}
