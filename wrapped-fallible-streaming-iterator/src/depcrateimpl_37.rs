// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl < I , F > FallibleStreamingIterator for TakeWhile < I , F > where I : FallibleStreamingIterator , F : FnMut (& I :: Item) -> bool , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn advance (& mut self) -> Result < () , I :: Error > { if let Some (v) = self . it . next () ? { if ! (self . f) (v) { self . done = true ; } } Ok (()) } # [inline] fn get (& self) -> Option < & I :: Item > { if self . done { None } else { self . it . get () } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { if self . done { (0 , Some (0)) } else { (0 , self . it . size_hint () . 1) } } }
};
}
