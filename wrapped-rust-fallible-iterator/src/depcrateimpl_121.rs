// Generated macro for impl_121 (impl)
macro_rules! Depcrateimpl_121 {
() => {
// Module: crate
// Provides: {"impl_121"}
// Dependencies: {}
impl < I > FallibleIterator for Skip < I > where I : FallibleIterator , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn next (& mut self) -> Result < Option < I :: Item > , I :: Error > { if self . n == 0 { self . it . next () } else { let n = self . n ; self . n = 0 ; self . it . nth (n) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let hint = self . it . size_hint () ; (hint . 0 . saturating_sub (self . n) , hint . 1 . map (| x | x . saturating_sub (self . n)) ,) } }
};
}
