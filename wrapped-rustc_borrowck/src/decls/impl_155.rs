macro_rules! deps {
    () => {
        ConstraintDescription!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < 'tcx > ConstraintDescription for ConstraintCategory < 'tcx > { fn description (& self) -> & 'static str { match self { ConstraintCategory :: Assignment => "assignment " , ConstraintCategory :: Return (_) => "returning this value " , ConstraintCategory :: Yield => "yielding this value " , ConstraintCategory :: UseAsConst => "using this value as a constant " , ConstraintCategory :: UseAsStatic => "using this value as a static " , ConstraintCategory :: Cast { is_implicit_coercion : false , .. } => "cast " , ConstraintCategory :: Cast { is_implicit_coercion : true , .. } => "coercion " , ConstraintCategory :: CallArgument (_) => "argument " , ConstraintCategory :: TypeAnnotation (AnnotationSource :: GenericArg) => "generic argument " , ConstraintCategory :: TypeAnnotation (_) => "type annotation " , ConstraintCategory :: SizedBound => "proving this value is `Sized` " , ConstraintCategory :: CopyBound => "copying this value " , ConstraintCategory :: OpaqueType => "opaque type " , ConstraintCategory :: ClosureUpvar (_) => "closure capture " , ConstraintCategory :: Usage => "this usage " , ConstraintCategory :: Predicate (_) | ConstraintCategory :: Boring | ConstraintCategory :: BoringNoLocation | ConstraintCategory :: Internal | ConstraintCategory :: OutlivesUnnameablePlaceholder (..) => "" , } } }
    };
}

impl_155!()