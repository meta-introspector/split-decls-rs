// Generated macro for is_stable_const_fn (function)
macro_rules! Depcrate_qualify_min_const_fnis_stable_const_fn {
() => {
// Module: crate::qualify_min_const_fn
// Provides: {"is_stable_const_fn"}
// Dependencies: {}
# [doc = " Checks if the given `def_id` is a stable const fn, in respect to the given MSRV."] pub fn is_stable_const_fn (cx : & LateContext < '_ > , def_id : DefId , msrv : Msrv) -> bool { cx . tcx . is_const_fn (def_id) && cx . tcx . lookup_const_stability (def_id) . or_else (| | { cx . tcx . trait_of_assoc (def_id) . and_then (| trait_def_id | cx . tcx . lookup_const_stability (trait_def_id)) }) . is_none_or (| const_stab | { if let rustc_hir :: StabilityLevel :: Stable { since , .. } = const_stab . level { let const_stab_rust_version = match since { StableSince :: Version (version) => version , StableSince :: Current => RustcVersion :: CURRENT , StableSince :: Err (_) => return false , } ; msrv . meets (cx , const_stab_rust_version) } else { cx . tcx . features () . enabled (const_stab . feature) && msrv . current (cx) . is_none () } }) }
};
}
