// Generated macro for impl_106 (impl)
macro_rules! Depcrateimpl_106 {
() => {
// Module: crate
// Provides: {"impl_106"}
// Dependencies: {}
impl < I > DoubleEndedIterator for Iterator < I > where I : DoubleEndedFallibleIterator , { # [inline] fn next_back (& mut self) -> Option < Result < I :: Item , I :: Error > > { match self . 0 . next_back () { Ok (Some (v)) => Some (Ok (v)) , Ok (None) => None , Err (e) => Some (Err (e)) , } } }
};
}
