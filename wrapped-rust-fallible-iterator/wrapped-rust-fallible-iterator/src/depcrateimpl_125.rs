// Generated macro for impl_125 (impl)
macro_rules! Depcrateimpl_125 {
() => {
// Module: crate
// Provides: {"impl_125"}
// Dependencies: {}
impl < I > FallibleIterator for StepBy < I > where I : FallibleIterator , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn next (& mut self) -> Result < Option < I :: Item > , I :: Error > { if self . first_take { self . first_take = false ; self . it . next () } else { self . it . nth (self . step) } } fn size_hint (& self) -> (usize , Option < usize >) { let inner_hint = self . it . size_hint () ; if self . first_take { let f = | n | { if n == 0 { 0 } else { 1 + (n - 1) / (self . step + 1) } } ; (f (inner_hint . 0) , inner_hint . 1 . map (f)) } else { let f = | n | n / (self . step + 1) ; (f (inner_hint . 0) , inner_hint . 1 . map (f)) } } }
};
}
