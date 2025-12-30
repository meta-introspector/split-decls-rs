// Generated macro for impl_151 (impl)
macro_rules! Depcrateimpl_151 {
() => {
// Module: crate
// Provides: {"impl_151"}
// Dependencies: {}
impl < T : Clone , E > FallibleIterator for Repeat < T , E > { type Item = T ; type Error = E ; # [inline] fn next (& mut self) -> Result < Option < Self :: Item > , Self :: Error > { Ok (Some (self . 0 . clone ())) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (usize :: max_value () , None) } }
};
}
