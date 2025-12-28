macro_rules! RelaxedBoundForbiddenReason {
    () => {
        # [derive (Clone , Copy , Debug)] enum RelaxedBoundForbiddenReason { TraitObjectTy , SuperTrait , AssocTyBounds , LateBoundVarsInScope , }
    };
}

RelaxedBoundForbiddenReason!()