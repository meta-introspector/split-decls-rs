// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl < I , F , B : ? Sized > FallibleStreamingIterator for MapRef < I , F > where I : FallibleStreamingIterator , F : Fn (& I :: Item) -> & B , { type Item = B ; type Error = I :: Error ; # [inline] fn advance (& mut self) -> Result < () , I :: Error > { self . it . advance () } # [inline] fn get (& self) -> Option < & B > { self . it . get () . map (& self . f) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
