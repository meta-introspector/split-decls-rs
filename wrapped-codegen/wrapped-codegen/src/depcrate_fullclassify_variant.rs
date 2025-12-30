// Generated macro for classify_variant (function)
macro_rules! Depcrate_fullclassify_variant {
() => {
// Module: crate::full
// Provides: {"classify_variant"}
// Dependencies: {}
fn classify_variant (defs : & Definitions , fields : & [Type]) -> VariantAvailability { let mut has_derive = false ; let mut has_full = false ; for field in fields { for_each_syn_type (field , & mut | ty | { let node = lookup :: node (defs , ty) ; let derive = node . features . any . contains ("derive") ; let full = node . features . any . contains ("full") ; match (derive , full) { (false , false) => { } (false , true) => has_full = true , (true , false | true) => has_derive = true , } }) ; } if has_full { VariantAvailability :: Full } else if has_derive { VariantAvailability :: Derive } else { VariantAvailability :: Other } }
};
}
