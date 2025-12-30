// Generated macro for is_mixed_derive_full_enum (function)
macro_rules! Depcrate_fullis_mixed_derive_full_enum {
() => {
// Module: crate::full
// Provides: {"is_mixed_derive_full_enum"}
// Dependencies: {}
# [doc = " Syntax tree enum that has some variants enabled in \"derive\" mode and the"] # [doc = " rest enabled in \"full\" mode."] pub fn is_mixed_derive_full_enum (defs : & Definitions , node : & Node) -> bool { if ! (node . features . any . contains ("derive") && node . features . any . contains ("full")) { return false ; } let variants = match & node . data { Data :: Enum (variants) => variants , Data :: Private | Data :: Struct (_) => return false , } ; let mut has_derive = false ; let mut has_full = false ; for fields in variants . values () { match classify_variant (defs , fields) { VariantAvailability :: Derive => has_derive = true , VariantAvailability :: Full => has_full = true , VariantAvailability :: Other => { } } } has_derive && has_full }
};
}
