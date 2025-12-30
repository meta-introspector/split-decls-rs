// Generated macro for impl_1867 (impl)
macro_rules! Depcrate_empty_enumsimpl_1867 {
() => {
// Module: crate::empty_enums
// Provides: {"impl_1867"}
// Dependencies: {}
impl LateLintPass < '_ > for EmptyEnums { fn check_item (& mut self , cx : & LateContext < '_ > , item : & Item < '_ >) { if let ItemKind :: Enum (.. , def) = item . kind && def . variants . is_empty () && cx . tcx . features () . never_type () && ! span_contains_cfg (cx , item . span) { span_lint_and_help (cx , EMPTY_ENUMS , item . span , "enum with no variants" , None , "consider using the uninhabited type `!` (never type) or a wrapper \
                around it to introduce a type which can't be instantiated" ,) ; } } }
};
}
