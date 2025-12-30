// Generated macro for get_relevant_span (function)
macro_rules! Depcrate_foreign_modulesget_relevant_span {
() => {
// Module: crate::foreign_modules
// Provides: {"get_relevant_span"}
// Dependencies: {}
# [doc = " We want to ensure that we use spans for both decls that include where the"] # [doc = " name was defined, whether that was from the link_name attribute or not."] fn get_relevant_span (tcx : TyCtxt < '_ > , fi : hir :: OwnerId) -> Span { match name_of_extern_decl (tcx , fi) { SymbolName :: Normal (_) => tcx . def_span (fi) , SymbolName :: Link (_ , annot_span) => annot_span , } }
};
}
