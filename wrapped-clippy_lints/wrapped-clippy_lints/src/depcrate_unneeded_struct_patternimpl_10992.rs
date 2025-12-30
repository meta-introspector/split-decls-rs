// Generated macro for impl_10992 (impl)
macro_rules! Depcrate_unneeded_struct_patternimpl_10992 {
() => {
// Module: crate::unneeded_struct_pattern
// Provides: {"impl_10992"}
// Dependencies: {}
impl LateLintPass < '_ > for UnneededStructPattern { fn check_pat (& mut self , cx : & LateContext < '_ > , pat : & Pat < '_ >) { if ! pat . span . from_expansion () && let PatKind :: Struct (path , [] , _) = & pat . kind && let QPath :: Resolved (_ , path) = path && let Res :: Def (DefKind :: Variant , did) = path . res { let enum_did = cx . tcx . parent (did) ; let variant = cx . tcx . adt_def (enum_did) . variant_with_id (did) ; let has_only_fields_brackets = variant . ctor . is_some () && variant . fields . is_empty () ; let non_exhaustive_activated = variant . field_list_has_applicable_non_exhaustive () ; if ! has_only_fields_brackets || non_exhaustive_activated { return ; } if is_from_proc_macro (cx , * path) { return ; } if let Some (brackets_span) = pat . span . trim_start (path . span) { span_lint_and_sugg (cx , UNNEEDED_STRUCT_PATTERN , brackets_span , "struct pattern is not needed for a unit variant" , "remove the struct pattern" , String :: new () , Applicability :: MachineApplicable ,) ; } } } }
};
}
