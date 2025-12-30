// Generated macro for check_legal_trait_for_method_call (function)
macro_rules! Depcrate_calleecheck_legal_trait_for_method_call {
() => {
// Module: crate::callee
// Provides: {"check_legal_trait_for_method_call"}
// Dependencies: {}
# [doc = " Checks that it is legal to call methods of the trait corresponding"] # [doc = " to `trait_id` (this only cares about the trait, not the specific"] # [doc = " method that is called)."] pub (crate) fn check_legal_trait_for_method_call (tcx : TyCtxt < '_ > , span : Span , receiver : Option < Span > , expr_span : Span , trait_id : DefId , _body_id : DefId ,) -> Result < () , ErrorGuaranteed > { if tcx . is_lang_item (trait_id , LangItem :: Drop) { let sugg = if let Some (receiver) = receiver . filter (| s | ! s . is_empty ()) { errors :: ExplicitDestructorCallSugg :: Snippet { lo : expr_span . shrink_to_lo () , hi : receiver . shrink_to_hi () . to (expr_span . shrink_to_hi ()) , } } else { errors :: ExplicitDestructorCallSugg :: Empty (span) } ; return Err (tcx . dcx () . emit_err (errors :: ExplicitDestructorCall { span , sugg })) ; } tcx . ensure_ok () . coherent_trait (trait_id) }
};
}
