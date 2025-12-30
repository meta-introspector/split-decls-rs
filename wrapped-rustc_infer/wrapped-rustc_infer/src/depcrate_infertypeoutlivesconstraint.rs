// Generated macro for TypeOutlivesConstraint (struct)
macro_rules! Depcrate_inferTypeOutlivesConstraint {
() => {
// Module: crate::infer
// Provides: {"TypeOutlivesConstraint"}
// Dependencies: {}
# [doc = " See the `region_obligations` field for more information."] # [derive (Clone , Debug)] pub struct TypeOutlivesConstraint < 'tcx > { pub sub_region : ty :: Region < 'tcx > , pub sup_type : Ty < 'tcx > , pub origin : SubregionOrigin < 'tcx > , }
};
}
