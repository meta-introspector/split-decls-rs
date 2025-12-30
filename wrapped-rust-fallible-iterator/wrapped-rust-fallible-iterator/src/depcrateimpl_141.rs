// Generated macro for impl_141 (impl)
macro_rules! Depcrateimpl_141 {
() => {
// Module: crate
// Provides: {"impl_141"}
// Dependencies: {}
impl < T , E > FallibleIterator for Empty < T , E > { type Item = T ; type Error = E ; # [inline] fn next (& mut self) -> Result < Option < T > , E > { Ok (None) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } }
};
}
