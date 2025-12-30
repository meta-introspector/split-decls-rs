// Generated macro for impl_105 (impl)
macro_rules! Depcrateimpl_105 {
() => {
// Module: crate
// Provides: {"impl_105"}
// Dependencies: {}
impl < I > iter :: Iterator for Iterator < I > where I : FallibleIterator , { type Item = Result < I :: Item , I :: Error > ; # [inline] fn next (& mut self) -> Option < Result < I :: Item , I :: Error > > { match self . 0 . next () { Ok (Some (v)) => Some (Ok (v)) , Ok (None) => None , Err (e) => Some (Err (e)) , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
};
}
