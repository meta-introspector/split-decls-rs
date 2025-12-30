// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl < I , F , B > DoubleEndedFallibleStreamingIterator for MapErr < I , F > where I : DoubleEndedFallibleStreamingIterator , F : Fn (I :: Error) -> B , { # [inline] fn advance_back (& mut self) -> Result < () , B > { self . it . advance_back () . map_err (& mut self . f) } # [inline] fn next_back (& mut self) -> Result < Option < & I :: Item > , B > { self . it . next_back () . map_err (& mut self . f) } }
};
}
