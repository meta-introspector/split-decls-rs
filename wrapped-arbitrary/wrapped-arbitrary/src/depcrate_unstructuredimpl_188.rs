// Generated macro for impl_188 (impl)
macro_rules! Depcrate_unstructuredimpl_188 {
() => {
// Module: crate::unstructured
// Provides: {"impl_188"}
// Dependencies: {}
impl < 'a , ElementType : Arbitrary < 'a > > Iterator for ArbitraryIter < 'a , '_ , ElementType > { type Item = Result < ElementType > ; fn next (& mut self) -> Option < Result < ElementType > > { let keep_going = self . u . arbitrary () . unwrap_or (false) ; if keep_going { Some (Arbitrary :: arbitrary (self . u)) } else { None } } }
};
}
