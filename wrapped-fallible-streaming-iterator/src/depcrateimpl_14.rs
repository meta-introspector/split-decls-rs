// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl < T , E > FallibleStreamingIterator for Empty < T , E > { type Item = T ; type Error = E ; # [inline] fn advance (& mut self) -> Result < () , E > { Ok (()) } # [inline] fn get (& self) -> Option < & T > { None } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } }
};
}
