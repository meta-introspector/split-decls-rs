// Generated macro for DefiningUse (struct)
macro_rules! Depcrate_region_infer_opaque_typesDefiningUse {
() => {
// Module: crate::region_infer::opaque_types
// Provides: {"DefiningUse"}
// Dependencies: {}
# [derive (Debug)] struct DefiningUse < 'tcx > { # [doc = " The opaque type using non NLL vars. This uses the actual"] # [doc = " free regions and placeholders. This is necessary"] # [doc = " to interact with code outside of `rustc_borrowck`."] opaque_type_key : OpaqueTypeKey < 'tcx > , arg_regions : Vec < RegionVid > , hidden_type : OpaqueHiddenType < 'tcx > , }
};
}
