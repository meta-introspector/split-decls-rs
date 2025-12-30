// Generated macro for impl_467 (impl)
macro_rules! Depcrate_diagnostics_region_errorsimpl_467 {
() => {
// Module: crate::diagnostics::region_errors
// Provides: {"impl_467"}
// Dependencies: {}
impl < 'tcx > ConstraintDescription for ConstraintCategory < 'tcx > { fn description (& self) -> & 'static str { match self { ConstraintCategory :: Assignment => "assignment " , ConstraintCategory :: Return (_) => "returning this value " , ConstraintCategory :: Yield => "yielding this value " , ConstraintCategory :: UseAsConst => "using this value as a constant " , ConstraintCategory :: UseAsStatic => "using this value as a static " , ConstraintCategory :: Cast { is_implicit_coercion : false , .. } => "cast " , ConstraintCategory :: Cast { is_implicit_coercion : true , .. } => "coercion " , ConstraintCategory :: CallArgument (_) => "argument " , ConstraintCategory :: TypeAnnotation (AnnotationSource :: GenericArg) => "generic argument " , ConstraintCategory :: TypeAnnotation (_) => "type annotation " , ConstraintCategory :: SizedBound => "proving this value is `Sized` " , ConstraintCategory :: CopyBound => "copying this value " , ConstraintCategory :: OpaqueType => "opaque type " , ConstraintCategory :: ClosureUpvar (_) => "closure capture " , ConstraintCategory :: Usage => "this usage " , ConstraintCategory :: Predicate (_) | ConstraintCategory :: Boring | ConstraintCategory :: BoringNoLocation | ConstraintCategory :: Internal | ConstraintCategory :: OutlivesUnnameablePlaceholder (..) => "" , } } }
};
}
