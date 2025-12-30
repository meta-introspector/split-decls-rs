// Generated macro for TyBuilder (struct)
macro_rules! Depcrate_builderTyBuilder {
() => {
// Module: crate::builder
// Provides: {"TyBuilder"}
// Dependencies: {}
# [doc = " This is a builder for `Ty` or anything that needs a `Substitution`."] pub struct TyBuilder < D > { # [doc = " The `data` field is used to keep track of what we're building (e.g. an"] # [doc = " ADT, a `TraitRef`, ...)."] data : D , vec : SmallVec < [GenericArg ; 2] > , param_kinds : SmallVec < [ParamKind ; 2] > , parent_subst : Substitution , }
};
}
