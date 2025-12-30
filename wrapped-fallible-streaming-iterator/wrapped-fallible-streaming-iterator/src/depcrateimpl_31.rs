// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl < I > FallibleStreamingIterator for Skip < I > where I : FallibleStreamingIterator , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn advance (& mut self) -> Result < () , I :: Error > { for _ in 0 .. self . n { if let None = self . it . next () ? { return Ok (()) ; } } self . n = 0 ; self . advance () } # [inline] fn get (& self) -> Option < & I :: Item > { self . it . get () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let hint = self . it . size_hint () ; (hint . 0 . saturating_sub (self . n) , hint . 1 . map (| h | h . saturating_sub (self . n)) ,) } }
};
}
