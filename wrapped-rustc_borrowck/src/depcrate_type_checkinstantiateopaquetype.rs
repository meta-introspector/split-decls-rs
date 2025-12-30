// Generated macro for InstantiateOpaqueType (struct)
macro_rules! Depcrate_type_checkInstantiateOpaqueType {
() => {
// Module: crate::type_check
// Provides: {"InstantiateOpaqueType"}
// Dependencies: {}
# [doc = " Runs `infcx.instantiate_opaque_types`. Unlike other `TypeOp`s,"] # [doc = " this is not canonicalized - it directly affects the main `InferCtxt`"] # [doc = " that we use during MIR borrowchecking."] # [derive (Debug)] pub (super) struct InstantiateOpaqueType < 'tcx > { pub base_universe : Option < ty :: UniverseIndex > , pub region_constraints : Option < RegionConstraintData < 'tcx > > , pub obligations : PredicateObligations < 'tcx > , }
};
}
