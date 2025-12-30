// Generated macro for RegionInferReason (enum)
macro_rules! Depcrate_hir_ty_loweringRegionInferReason {
() => {
// Module: crate::hir_ty_lowering
// Provides: {"RegionInferReason"}
// Dependencies: {}
# [derive (Debug)] pub enum RegionInferReason < 'a > { # [doc = " Lifetime on a trait object that is spelled explicitly, e.g. `+ 'a` or `+ '_`."] ExplicitObjectLifetime , # [doc = " A trait object's lifetime when it is elided, e.g. `dyn Any`."] ObjectLifetimeDefault , # [doc = " Generic lifetime parameter"] Param (& 'a ty :: GenericParamDef) , RegionPredicate , Reference , OutlivesBound , }
};
}
