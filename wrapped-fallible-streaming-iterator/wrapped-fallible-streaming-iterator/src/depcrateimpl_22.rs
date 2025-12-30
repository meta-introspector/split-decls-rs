// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl < I , F , B > FallibleStreamingIterator for Map < I , F , B > where I : FallibleStreamingIterator , F : FnMut (& I :: Item) -> B , { type Item = B ; type Error = I :: Error ; # [inline] fn advance (& mut self) -> Result < () , I :: Error > { self . value = self . it . next () ? . map (& mut self . f) ; Ok (()) } # [inline] fn get (& self) -> Option < & B > { self . value . as_ref () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
