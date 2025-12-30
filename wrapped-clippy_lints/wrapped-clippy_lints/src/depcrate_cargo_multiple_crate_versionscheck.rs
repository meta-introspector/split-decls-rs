// Generated macro for check (function)
macro_rules! Depcrate_cargo_multiple_crate_versionscheck {
() => {
// Module: crate::cargo::multiple_crate_versions
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , metadata : & Metadata , allowed_duplicate_crates : & FxHashSet < String >) { let local_name = cx . tcx . crate_name (LOCAL_CRATE) ; let mut packages = metadata . packages . clone () ; packages . sort_by (| a , b | a . name . cmp (& b . name)) ; if let Some (resolve) = & metadata . resolve && let Some (local_id) = packages . iter () . find_map (| p | { if p . name . as_bytes () . iter () . map (| b | if b == & b'-' { & b'_' } else { b }) . eq (local_name . as_str () . as_bytes ()) { Some (& p . id) } else { None } }) { for (name , group) in & packages . iter () . filter (| p | ! allowed_duplicate_crates . contains (& p . name)) . group_by (| p | & p . name) { let group : Vec < & Package > = group . collect () ; if group . len () <= 1 { continue ; } if group . iter () . all (| p | is_normal_dep (& resolve . nodes , local_id , & p . id)) { let mut versions : Vec < _ > = group . into_iter () . map (| p | & p . version) . collect () ; versions . sort () ; let versions = versions . iter () . join (", ") ; span_lint (cx , MULTIPLE_CRATE_VERSIONS , DUMMY_SP , format ! ("multiple versions for dependency `{name}`: {versions}") ,) ; } } } }
};
}
