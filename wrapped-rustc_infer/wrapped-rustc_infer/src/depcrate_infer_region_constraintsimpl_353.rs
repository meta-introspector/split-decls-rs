// Generated macro for impl_353 (impl)
macro_rules! Depcrate_infer_region_constraintsimpl_353 {
() => {
// Module: crate::infer::region_constraints
// Provides: {"impl_353"}
// Dependencies: {}
impl < 'tcx > fmt :: Display for GenericKind < 'tcx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { GenericKind :: Param (ref p) => write ! (f , "{p}") , GenericKind :: Placeholder (ref p) => write ! (f , "{p}") , GenericKind :: Alias (ref p) => write ! (f , "{p}") , } } }
};
}
