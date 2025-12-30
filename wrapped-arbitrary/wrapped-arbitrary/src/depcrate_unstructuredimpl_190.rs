// Generated macro for impl_190 (impl)
macro_rules! Depcrate_unstructuredimpl_190 {
() => {
// Module: crate::unstructured
// Provides: {"impl_190"}
// Dependencies: {}
impl < 'a , ElementType : Arbitrary < 'a > > Iterator for ArbitraryTakeRestIter < 'a , ElementType > { type Item = Result < ElementType > ; fn next (& mut self) -> Option < Result < ElementType > > { let keep_going = self . u . arbitrary () . unwrap_or (false) ; if keep_going { Some (Arbitrary :: arbitrary (& mut self . u)) } else { None } } }
};
}
