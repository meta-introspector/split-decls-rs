// Generated macro for impl_352 (impl)
macro_rules! Depcrate_infer_region_constraintsimpl_352 {
() => {
// Module: crate::infer::region_constraints
// Provides: {"impl_352"}
// Dependencies: {}
impl < 'tcx > fmt :: Debug for GenericKind < 'tcx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { GenericKind :: Param (ref p) => write ! (f , "{p:?}") , GenericKind :: Placeholder (ref p) => write ! (f , "{p:?}") , GenericKind :: Alias (ref p) => write ! (f , "{p:?}") , } } }
};
}
