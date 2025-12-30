// Generated macro for impl_117 (impl)
macro_rules! Depcrateimpl_117 {
() => {
// Module: crate
// Provides: {"impl_117"}
// Dependencies: {}
impl < I > DoubleEndedFallibleIterator for Rev < I > where I : DoubleEndedFallibleIterator , { # [inline] fn next_back (& mut self) -> Result < Option < I :: Item > , I :: Error > { self . 0 . next () } # [inline] fn try_rfold < B , E , F > (& mut self , init : B , f : F) -> Result < B , E > where E : From < I :: Error > , F : FnMut (B , I :: Item) -> Result < B , E > , { self . 0 . try_fold (init , f) } }
};
}
