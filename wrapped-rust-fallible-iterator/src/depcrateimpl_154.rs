// Generated macro for impl_154 (impl)
macro_rules! Depcrateimpl_154 {
() => {
// Module: crate
// Provides: {"impl_154"}
// Dependencies: {}
impl < T , E : Clone > FallibleIterator for RepeatErr < T , E > { type Item = T ; type Error = E ; # [inline] fn next (& mut self) -> Result < Option < Self :: Item > , Self :: Error > { Err (self . 1 . clone ()) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } }
};
}
