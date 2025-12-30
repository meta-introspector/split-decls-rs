// Generated macro for DynCompatibilityViolation (enum)
macro_rules! Depcrate_dyn_compatibilityDynCompatibilityViolation {
() => {
// Module: crate::dyn_compatibility
// Provides: {"DynCompatibilityViolation"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum DynCompatibilityViolation { SizedSelf , SelfReferential , Method (FunctionId , MethodViolationCode) , AssocConst (ConstId) , GAT (TypeAliasId) , HasNonCompatibleSuperTrait (TraitId) , }
};
}
