// Generated macro for PlaceholderIndices (struct)
macro_rules! Depcrate_region_infer_valuesPlaceholderIndices {
() => {
// Module: crate::region_infer::values
// Provides: {"PlaceholderIndices"}
// Dependencies: {}
# [doc = " Maps from `ty::PlaceholderRegion` values that are used in the rest of"] # [doc = " rustc to the internal `PlaceholderIndex` values that are used in"] # [doc = " NLL."] # [derive (Debug , Default)] # [derive (Clone)] pub (crate) struct PlaceholderIndices { indices : FxIndexSet < ty :: PlaceholderRegion > , }
};
}
