// Generated macro for show_lint (function)
macro_rules! Depcrate_inherent_to_stringshow_lint {
() => {
// Module: crate::inherent_to_string
// Provides: {"show_lint"}
// Dependencies: {}
fn show_lint (cx : & LateContext < '_ > , item : & ImplItem < '_ >) { let display_trait_id = cx . tcx . get_diagnostic_item (sym :: Display) . expect ("Failed to get trait ID of `Display`!") ; let self_type = cx . tcx . fn_sig (item . owner_id) . skip_binder () . input (0) ; let self_type = self_type . skip_binder () . peel_refs () ; if implements_trait (cx , self_type , display_trait_id , & []) { span_lint_and_help (cx , INHERENT_TO_STRING_SHADOW_DISPLAY , item . span , format ! ("type `{self_type}` implements inherent method `to_string(&self) -> String` which shadows the implementation of `Display`") , None , format ! ("remove the inherent method from type `{self_type}`") ,) ; } else { span_lint_and_help (cx , INHERENT_TO_STRING , item . span , format ! ("implementation of inherent method `to_string(&self) -> String` for type `{self_type}`") , None , format ! ("implement trait `Display` for type `{self_type}` instead") ,) ; } }
};
}
