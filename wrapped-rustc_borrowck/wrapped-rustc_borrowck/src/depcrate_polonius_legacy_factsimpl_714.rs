// Generated macro for impl_714 (impl)
macro_rules! Depcrate_polonius_legacy_factsimpl_714 {
() => {
// Module: crate::polonius::legacy::facts
// Provides: {"impl_714"}
// Dependencies: {}
# [extension (pub (crate) trait PoloniusFactsExt)] impl PoloniusFacts { # [doc = " Returns `true` if there is a need to gather `PoloniusFacts` given the"] # [doc = " current `-Z` flags."] fn enabled (tcx : TyCtxt < '_ >) -> bool { tcx . sess . opts . unstable_opts . nll_facts || tcx . sess . opts . unstable_opts . polonius . is_legacy_enabled () } fn write_to_dir (& self , dir : impl AsRef < Path > , location_table : & PoloniusLocationTable ,) -> Result < () , Box < dyn Error > > { let dir : & Path = dir . as_ref () ; fs :: create_dir_all (dir) ? ; let wr = FactWriter { location_table , dir } ; macro_rules ! write_facts_to_path { ($ wr : ident . write_facts_to_path ($ this : ident . [$ ($ field : ident ,) *])) => { $ ($ wr . write_facts_to_path (&$ this .$ field , & format ! ("{}.facts" , stringify ! ($ field))) ?;) * } } write_facts_to_path ! { wr . write_facts_to_path (self . [loan_issued_at , universal_region , cfg_edge , loan_killed_at , subset_base , loan_invalidated_at , var_used_at , var_defined_at , var_dropped_at , use_of_var_derefs_origin , drop_of_var_derefs_origin , child_path , path_is_var , path_assigned_at_base , path_moved_at_base , path_accessed_at_base , known_placeholder_subset , placeholder ,]) } Ok (()) } }
};
}
