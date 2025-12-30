// Generated macro for GenericKind (enum)
macro_rules! Depcrate_infer_region_constraintsGenericKind {
() => {
// Module: crate::infer::region_constraints
// Provides: {"GenericKind"}
// Dependencies: {}
# [derive (Copy , Clone , PartialEq , Eq , Hash , TypeFoldable , TypeVisitable)] pub enum GenericKind < 'tcx > { Param (ty :: ParamTy) , Placeholder (ty :: PlaceholderType) , Alias (ty :: AliasTy < 'tcx >) , }
};
}
