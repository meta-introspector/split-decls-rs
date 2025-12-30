// Generated macro for emit_access_facts (function)
macro_rules! Depcrate_polonius_legacy_accessesemit_access_facts {
() => {
// Module: crate::polonius::legacy::accesses
// Provides: {"emit_access_facts"}
// Dependencies: {}
# [doc = " Emit polonius facts for variable defs, uses, drops, and path accesses."] pub (crate) fn emit_access_facts < 'tcx > (tcx : TyCtxt < 'tcx > , facts : & mut PoloniusFacts , body : & Body < 'tcx > , location_table : & PoloniusLocationTable , move_data : & MoveData < 'tcx > , universal_regions : & UniversalRegions < 'tcx > ,) { let mut extractor = AccessFactsExtractor { facts , move_data , location_table } ; extractor . visit_body (body) ; for (local , local_decl) in body . local_decls . iter_enumerated () { debug ! ("add use_of_var_derefs_origin facts - local={:?}, type={:?}" , local , local_decl . ty) ; tcx . for_each_free_region (& local_decl . ty , | region | { let region_vid = universal_regions . to_region_vid (region) ; facts . use_of_var_derefs_origin . push ((local , region_vid . into ())) ; }) ; } }
};
}
