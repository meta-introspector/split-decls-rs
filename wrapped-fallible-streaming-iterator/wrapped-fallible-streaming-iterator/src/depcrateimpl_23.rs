// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl < I , F , B > DoubleEndedFallibleStreamingIterator for Map < I , F , B > where I : DoubleEndedFallibleStreamingIterator , F : FnMut (& I :: Item) -> B , { # [inline] fn advance_back (& mut self) -> Result < () , I :: Error > { self . value = self . it . next_back () ? . map (& mut self . f) ; Ok (()) } }
};
}
