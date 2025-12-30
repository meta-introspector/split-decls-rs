// Generated macro for impl_1840 (impl)
macro_rules! Depcrate_empty_enumimpl_1840 {
() => {
// Module: crate::empty_enum
// Provides: {"impl_1840"}
// Dependencies: {}
impl LateLintPass < '_ > for EmptyEnum { fn check_item (& mut self , cx : & LateContext < '_ > , item : & Item < '_ >) { if let ItemKind :: Enum (..) = item . kind && cx . tcx . features () . never_type () && let Some (adt) = cx . tcx . type_of (item . owner_id) . instantiate_identity () . ty_adt_def () && adt . variants () . is_empty () { span_lint_and_help (cx , EMPTY_ENUM , item . span , "enum with no variants" , None , "consider using the uninhabited type `!` (never type) or a wrapper \
                around it to introduce a type which can't be instantiated" ,) ; } } }
};
}
