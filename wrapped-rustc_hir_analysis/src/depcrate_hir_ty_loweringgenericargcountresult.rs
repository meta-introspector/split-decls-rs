// Generated macro for GenericArgCountResult (struct)
macro_rules! Depcrate_hir_ty_loweringGenericArgCountResult {
() => {
// Module: crate::hir_ty_lowering
// Provides: {"GenericArgCountResult"}
// Dependencies: {}
# [doc = " Decorates the result of a generic argument count mismatch"] # [doc = " check with whether explicit late bounds were provided."] # [derive (Clone , Debug)] pub struct GenericArgCountResult { pub explicit_late_bound : ExplicitLateBound , pub correct : Result < () , GenericArgCountMismatch > , }
};
}
