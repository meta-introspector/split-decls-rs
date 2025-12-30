// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
impl < I , F > FallibleStreamingIterator for SkipWhile < I , F > where I : FallibleStreamingIterator , F : FnMut (& I :: Item) -> bool , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn advance (& mut self) -> Result < () , I :: Error > { if ! self . done { self . done = true ; let f = & mut self . f ; self . it . find (| i | ! f (i)) . map (| _ | ()) } else { self . it . advance () } } # [inline] fn get (& self) -> Option < & I :: Item > { self . it . get () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let hint = self . it . size_hint () ; if self . done { hint } else { (0 , hint . 1) } } }
};
}
