macro_rules! InstantiateOpaqueType {
    () => {
        # [doc = " Runs `infcx.instantiate_opaque_types`. Unlike other `TypeOp`s,"] # [doc = " this is not canonicalized - it directly affects the main `InferCtxt`"] # [doc = " that we use during MIR borrowchecking."] # [derive (Debug)] pub (super) struct InstantiateOpaqueType < 'tcx > { pub base_universe : Option < ty :: UniverseIndex > , pub region_constraints : Option < RegionConstraintData < 'tcx > > , pub obligations : PredicateObligations < 'tcx > , }
    };
}

InstantiateOpaqueType!();