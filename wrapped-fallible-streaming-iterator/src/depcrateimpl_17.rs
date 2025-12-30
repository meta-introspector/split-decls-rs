// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl < I , F > FallibleStreamingIterator for Filter < I , F > where I : FallibleStreamingIterator , F : FnMut (& I :: Item) -> bool , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn advance (& mut self) -> Result < () , I :: Error > { while let Some (i) = self . it . next () ? { if (self . f) (i) { break ; } } Ok (()) } # [inline] fn get (& self) -> Option < & I :: Item > { self . it . get () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , self . it . size_hint () . 1) } }
};
}
